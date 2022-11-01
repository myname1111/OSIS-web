use common::Date;
use yew::prelude::*;
use crate::backend::*;

#[function_component(MemberComp)]
pub fn member_comp(props: &MemberProp) -> Html {
    let member_id = props.member_id;
    let member = use_state(|| None);

    {
        let member = member.clone();
        use_effect_with_deps( move |_| {
            let member = member.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_member = get_member(member_id).await;
                member.set(Some(fetched_member));
            });
            || ()
        }, ())
    }

    let member_profile: Option<u32> = member.as_ref().and_then(|member |
        member.profile.map(|id| id.try_into().unwrap())
    );

    if let Some(member) = &*member {
       html! {
            <div>
                <MemberInfo name={ member.name.clone() } 
                    division={ member.division } joined={ member.joined }
                    class={ member.class.clone() } profile={ member_profile }/>
                <MemberDesc bio={ member.bio.clone() } />
            </div>
        } 
    } else {
        html! {
            <>
                <p>{ "Loading, please wait" }</p>
            </>
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct MemberProp {
    pub member_id: u32,
}

#[function_component(MemberInfo)]
fn member_info(props: &MemberInfoProp) -> Html {
    let image = use_state(|| None);

    if let Some(id) = props.profile {
        let image = image.clone();
        use_effect_with_deps(move |_| {
            let image = image.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_image = get_image(id).await;

                image.set(Some(fetched_image))
            });
            || ()
        }, ())
    };


    html! {
        <div class="flex margin-vert-80">
            <div class="width-40 flex center-horz space-around center-horz">
                <div class="width-80">
                    <img src={
                        if let Some(image) = &*image {
                            format!("http://localhost/data/{}", image.path)
                        } else {
                            "http://localhost/data/person.png".to_string()
                        }
                    }
                        alt="member profile"/> // Replace alt with {member.name}
                </div>
            </div> 
            <div class="member-properties">
                <h2>
                    {format!("name: {}", props.name.clone())}
                </h2>
                <h2>
                    {format!("division {}", props.division.unwrap_or(0))}
                </h2>
                <h2>
                    {format!("date joined: {}", props.joined.clone())}
                </h2>
                <h2>
                    {format!("class: {}", props.class.clone())}
                </h2>
                <a href="https://0.0.0.0/report"
                    class="link-nochange font-large"
                > // TODO: make
                                                                                   // report
                    {"Report"}
                </a>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MemberInfoProp {
    name: String,
    division: Option<i32>,
    joined: Date,
    class: String,
    profile: Option<u32>
}
  
#[function_component(MemberDesc)]
fn member_desc(props: &MemberDescProp) -> Html { 
    html! {
        <div class="flex list-vert margin-small">
            <p>{ props.bio.clone() }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MemberDescProp {
    bio: String,
}
