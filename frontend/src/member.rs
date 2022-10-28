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
    let id = props.profile.unwrap_or(0);
    let image = use_state(|| None);

    {
        let image = image.clone();
        use_effect_with_deps(move |_| {
            let image = image.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_image = get_image(id.try_into().unwrap()).await;

                image.set(Some(fetched_image))
            });
            || ()
        }, ())
    }


    html! {
        <div class="grid vert-split-2-3">
            <img src={
                    if let Some(image) = &*image {
                        format!("http://localhost/data/{}", image.path)
                    } else {
                        "http://localhost/data/person.png".to_string()
                    }
                }
                alt="member profile"/>
            <div class="flex list-vert"> <h1 class="font-large">
                    {format!("name: {}", props.name.clone())}
                </h1>
                <h1 class="font-large">
                    {format!("division {}", props.division.unwrap_or(0))}
                </h1>
                 <h1 class="font-large">
                    {format!("date joined: {}", props.joined.clone())}
                </h1>
                <h1 class="font-large">
                    {format!("class: {}", props.class.clone())}
                </h1>
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
        <div class="flex list-vert">
            <p>{ props.bio.clone() }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MemberDescProp {
    bio: String,
}
