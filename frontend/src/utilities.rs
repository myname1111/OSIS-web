use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar(props: &NavBarProp) -> Html {
    let pos = match props.pos {
        NavBarPos::Fixed => "fixed",
        NavBarPos::Static => "static",
    };

    html! {
        <nav style={format!("position: {}", pos)}>
            <a href="/">
                <img src="/data/OSIS.png" class="width-auto height-100" />
            </a>
            <h2>{ "Why" }</h2>
            <h2>{ "About" }</h2>
            <h2>{ "Members" }</h2>
            <h2>{ "Programs" }</h2>
            <h2>{ "Events" }</h2>
            <h2 class="nav-sign-up"> { "Sign in" }</h2>
        </nav>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct NavBarProp {
    pub pos: NavBarPos,
}

#[derive(PartialEq, Eq)]
pub enum NavBarPos {
    Fixed,
    Static,
}

