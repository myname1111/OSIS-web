use crate::backend::*;
use crate::utilities::*;
use common::{Division, Image, Role::*};
use yew::prelude::*;
use yew_router::prelude::*;

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[function_component(MemberList)]
pub fn member_list() -> Html {
    let member_list = use_state(|| None);
    let division = use_state(|| None);

    {
        let division = division.clone();
        let member_list = member_list.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_list = get_member_list().await;
                    let mut fetched_division = vec![];

                    // Sort by alphabetical order of name
                    fetched_list
                        .sort_unstable_by(|member_1, member_2| member_2.name.cmp(&member_1.name));

                    // Create a list with only the members and the head of the divisions
                    let member_head_only = fetched_list.clone();

                    let member_head_only = member_head_only
                        .iter()
                        .filter(|&member| member.division.is_some())
                        .collect::<Vec<_>>();

                    let mut member_head_only = member_head_only
                        .iter()
                        .map(|&member| member.clone())
                        .collect::<Vec<_>>();

                    // Remove all members and heads
                    let fetched_list_ref = fetched_list
                        .iter()
                        .filter(|&member| member.division.is_none())
                        .collect::<Vec<_>>();

                    fetched_list = fetched_list_ref
                        .iter()
                        .map(|&member| member.clone())
                        .collect::<Vec<_>>();

                    // Sort by division
                    member_head_only.sort_by(|member_1, member_2| {
                    let msg = "Expecting here is safe as we SHOULD have filtered out
                        any members/heads with no division id, making it impossible for this to happen";
                    member_2.division.expect(msg)
                        .cmp(&member_1.division.expect(msg))
                });

                    // Add back member/heads
                    fetched_list.append(&mut member_head_only);

                    // Sort by their role
                    fetched_list.sort_by(|member_1, member_2| member_2.role.cmp(&member_1.role)); // TODO: Support sorting by other things

                    for member in &fetched_list {
                        fetched_division.push(match member.division {
                            Some(division) => {
                                Some(get_division(division.try_into().unwrap()).await)
                            }
                            None => None,
                        })
                    }

                    division.set(Some(fetched_division));
                    member_list.set(Some(fetched_list))
                });
                || ()
            },
            (),
        )
    }

    let images = use_state(|| None);

    if let Some(member_list) = &*member_list {
        let images = images.clone();
        let member_list = member_list.clone();
        use_effect_with_deps(
            move |_| {
                let images = images.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_images = vec![];
                    for member in member_list {
                        if let Some(profile) = member.profile {
                            fetched_images.push(Some(get_image(profile.try_into().unwrap()).await))
                        } else {
                            fetched_images.push(None)
                        }
                    }
                    images.set(Some(fetched_images))
                });
                || ()
            },
            (),
        )
    }

    html! {
        <>
            <NavBar />
            <div class="member-list">
                {
                    if let (Some(images), Some(division), Some(member_list)) = (&*images, &*division, &*member_list) {
                        log::debug!("images: {:?}", images);
                        log::debug!("members: {:?}", member_list);
                        member_list
                            .iter()
                            .zip(images.iter())
                            .zip(division.iter())
                            .map(|((member, image), division)| {
                                html! {
                                    <Member member={(*member).clone()} image={(*image).clone()} division={(*division).clone()}/>
                                }
                            }).collect::<Html>()
                    } else if let (None, None, Some(member_list)) = (&*images, &*division, &*member_list) {
                        member_list.iter().map(|member| {
                            html! {
                                <Member member={(*member).clone()} image={None} division={None} />
                            }
                        }).collect::<Html>()
                    } else {
                        html! {
                            <p>{"Loading, please wait"}</p>
                        }
                    }
                }
            </div>
        </>
    }
}

#[function_component(Member)]
fn member(props: &MemberProp) -> Html {
    log::info!("rendering preview with data {:?}", props.member);
    html! {
            <div class="member-list-item">
                <Link<Route> to={Route::Member {id: props.member.id.try_into().unwrap()}}
                    classes="member-list-item--link">
                <img src={format!("http://localhost/data/{}", props.image.as_ref()
                    .map(|image| image.path.clone())
                    .unwrap_or_else(|| "person.png".to_string()))} />
    <<<<<<< HEAD
                <h2>{ props.member.name.clone() }</h2>
    =======
    >>>>>>> backend
                <h2 class="member-list-item--role">{ capitalize_first_letter( {
                    let division_msg = props.division.as_ref()
                        .map(|division| format!(" of the division of {}", division.name))
                        .unwrap_or_else(|| "".to_string());

                    &match props.member.role {
                        Member => format!("Member{}", division_msg),
                        Head => format!("Head{}", division_msg),
                        role => String::from(role)
                    }
                }) }</h2>
                </ Link<Route>>
            </div>
        }
}

#[derive(Properties, PartialEq, Eq)]
struct MemberProp {
    member: common::MemberPreview,
    image: Option<Image>,
    division: Option<Division>,
}
