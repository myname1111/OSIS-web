use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="nav">
            <a href="/">
                <img src="/data/OSIS.png" class="nav-img" />
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
