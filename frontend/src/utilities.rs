use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="nav">
            <a href="/">
                <img src="/data/OSIS.png" class="nav--img" />
            </a>
            <a href="#"><h2>{ "Blog" }</h2></a>
            <a href="http://localhost/about/0"><h2>{ "About" }</h2></a>
            <a href="http://localhost/member"><h2>{ "Members" }</h2></a>
            <a href="#"><h2>{ "Programs" }</h2></a>
            <a href="#"><h2>{ "Events" }</h2></a>
            <h2 class="nav--sign-up"> { "Sign in" }</h2>
        </nav>
    }
}

#[function_component(SignUpButton)]
pub fn sign_up_button(props: &SignUpButtonProps) -> Html {
    html! {
        <div class={format!("{} {}", props.class.clone(), props.modifiers.clone())}>
            <p class="sign-up-button--text">{ "Sign up for a better future" }</p>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct SignUpButtonProps {
    #[prop_or_default]
    pub modifiers: String,
    #[prop_or("sign-up-button sign-up-button--hover".to_string())]
    pub class: String,
}

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    // TODO: create 404
    #[at("/")]
    Home,
    #[at("/about/:id")]
    About { id: u8 },
    #[at("/landing")]
    Landing,
    #[at("/member/:id")]
    Member { id: u32 },
    #[at("/member")]
    MemberList,
}
