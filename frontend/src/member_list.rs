use common::Image;
use yew::prelude::*;
use crate::backend::*;

#[function_component(MemberList)]
pub fn member_list() -> Html {
    let member_list = use_state(|| None);
    
    {
        let member_list = member_list.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local( async move {
                let mut fetched_list = get_member_list().await;
                fetched_list.sort_by(|member_1, member_2| 
                    member_2.role.cmp(&member_1.role)); // TODO: Support sorting by other things
                member_list.set(Some(fetched_list))
            });
            || ()
        }, ())
    }

    let images = use_state(|| None);

    if let Some(member_list) = &*member_list {
        let images = images.clone();
        let member_list = member_list.clone();
        use_effect_with_deps(move |_| {
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
        }, ())
    }

    html! {
        <div class="member-container">
            {
                if let (Some(images), Some(member_list)) = (&*images, &*member_list) {
                    log::debug!("images: {:?}", images);
                    log::debug!("members: {:?}", member_list);
                    member_list.iter().zip(images.iter()).map(|(member, image)| {
                        html! {
                            <Member member={(*member).clone()} image={(*image).clone()} />
                        }
                    }).collect::<Html>()
                } else if let (None, Some(member_list)) = (&*images, &*member_list) {
                    member_list.iter().map(|member| {
                        html! {
                            <Member member={(*member).clone()} image={None} />
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        <p>{"Loading, please wait"}</p>
                    }
                }
            }
        </div>
    }
}

#[function_component(Member)]
fn member(props: &MemberProp) -> Html {
    log::info!("rendering preview with data {:?}", props.member);
    html! {
        <div class="member">
            <h2 class="member-name">{ props.member.name.clone() }</h2>
            <img src={format!("http://localhost/data/{}", props.image.as_ref()
                .map(|image| image.path.clone())
                .unwrap_or_else(|| "person.png".to_string()))} /> 
            <h2 class="member-role">{ props.member.role }</h2>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
struct MemberProp {
    member: common::MemberPreview,
    image: Option<Image>
}
