use yew::prelude;
use common::Member;

fn get_member(id: u32) -> Member {
    todo!()
}

#[function_component(Member)]
fn member() -> Html {
    html! {
        <MemberInfo />
        <MemberDesc />
    }
}

#[function_component(MemberInfo)]
fn member_info() -> Html {
    let member: Member = get_member(todo);

    html! {
        <div class="grid vert-split-2-3">
            <img src={format!("https://0.0.0.0/api/images/{}", member.profile) alt="member profile"/>
            <div class="flex list-vert">
                <h1 class="font-large">
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
                <a href="https://0.0.0.0/report" class="link-nochange font-large"> // TODO: make
                                                                                   // report
                    {"Report"}
                </a>
        </div>
    }
}

#[function_component(MemberDesc)]
fn member_desc() -> Html {
    let member: Member = get_member(todo);

    html! {
        <div class="flex list-vert">
            <p>{ member.bio }</p>
            <div class="flex">
                {
                    for photo in member.photos.iter() {
                        <img src={fomat!("https://0.0.0.0/api/images/{}", photo) />
                    }
                }
            <div>
        </div>
    }
}
