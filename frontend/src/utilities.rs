use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar(props: &NavBarProp) -> Html {
    let pos = match props.pos {
        NavBarPos::Fixed => "fixed",
        NavBarPos::Static => "",
    };

    html! {
        <nav class={format!("{} back-base flex width-100 height-10 gap-20 top-0", pos)}>
            <a href="/">
                <img src="/data/OSIS.png" class="width-auto height-100" />
            </a>
            <h2>{ "Why" }</h2>
            <h2>{ "About" }</h2>
            <h2>{ "Members" }</h2>
            <h2>{ "Programs" }</h2>
            <h2 class="margin-right-auto">{ "Events" }</h2>
            <h2 class="margin-right-30"> { "Sign in" }</h2>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavBarProp {
    pub pos: NavBarPos,
}

#[derive(PartialEq)]
pub enum NavBarPos {
    Fixed,
    Static,
}

