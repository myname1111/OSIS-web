use common::Member;
use reqwasm::http::Request;
use yew::prelude::*;

fn get_member(id: u32) -> Member {
    try_get_member(id).expect("Cannot get member")
}

fn try_get_member(id: u32) -> Option<Member> {
    let member: UseStateHandle<Option<Member>> = use_state(|| None);
    let member_clone = member.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let member_fetch = Request::get(
                format!("https://0.0.0.0/api/member/{}", id).as_str()
            )
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        member.set(member_fetch)
    });
    match member_clone.as_ref() {
        Some(e) => Some(e.clone()),
        None => None
    }
}

#[function_component(MemberComp)]
pub fn member_comp(props: &MemberProp) -> Html {
    html! {
        <div>
            <MemberInfo member_id={ props.member_id } />
            <MemberDesc member_id={ props.member_id }/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MemberProp {
    pub member_id: u32,
}

#[function_component(MemberInfo)]
fn member_info(props: &MemberProp) -> Html {
    let member: Member = get_member(props.member_id);

    html! {
        <div class="grid vert-split-2-3">
            <img src={
                    format!("https://0.0.0.0/api/images/{}", member.profile
                            .unwrap_or(0))
                }
                alt="member profile"/>
            <div class="flex list-vert"> <h1 class="font-large">
                    {format!("name: {}", member.name)}
                </h1>
                <h1 class="font-large">
                    {format!("name: {}", member.name)}
                </h1>
                 <h1 class="font-large">
                    {format!("name: {}", member.name)}
                </h1>
                <h1 class="font-large">
                    {format!("name: {}", member.name)}
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

#[function_component(MemberDesc)]
fn member_desc(props: &MemberProp) -> Html {
    let member: Member = get_member(props.member_id);

    html! {
        <div class="flex list-vert">
            <p>{ member.bio }</p>
        </div>
    }
}
