use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="nav">
            <a href="/">
                <img src="/data/OSIS.png" class="nav--img" />
            </a>
            <h2>{ "Why" }</h2>
            <h2>{ "About" }</h2>
            <h2>{ "Members" }</h2>
            <h2>{ "Programs" }</h2>
            <h2>{ "Events" }</h2>
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
    pub class: String
}
