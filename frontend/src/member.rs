use common::{Member, Date};
use reqwasm::http::Request;
use yew::prelude::*;

async fn get_member(id: u32) -> Member {
    try_get_member(id).await.expect("Cannot get member")
}

async fn try_get_member(id: u32) -> Option<Member> {
    log::info!("Using id {}", id);
    let url = format!("http://localhost/api/member/{}", id);
    log::debug!("With url of {}", url);
    let member: Option<Member> = Request::get(url.as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    log::info!("{}", if member.is_some() { "Success!" } else { "No member found" });
    log::debug!("{:?}", member);
    member
}

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

    if let Some(member) = &*member {
       html! {
            <div>
                <MemberInfo profile={ member.profile } 
                    name={ member.name.clone() } division={ member.division }
                    joined={ member.joined } class={ member.class.clone() }/>
                <MemberDesc bio={ member.bio.clone() }/>
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
    html! {
        <div class="grid vert-split-2-3">
            <img src={
                    format!("https://0.0.0.0/api/images/{}", 
                            props.profile.unwrap_or(0))
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
    profile: Option<i32>,
    name: String,
    division: Option<i32>,
    joined: Date,
    class: String
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
    bio: String
}
