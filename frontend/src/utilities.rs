use common::{SignInData, SignInResponse};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::backend::send_sign_in_creds;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let has_sign_in = use_state(|| {
        let window = window().expect("Unable to get window");
        let storage = window
            .session_storage()
            .unwrap()
            .expect("Unable to get storage");

        storage
            .get_item("username")
            .expect("Unable to get possibly defined username")
            .is_some()
            & storage
                .get_item("password")
                .expect("Unable to get possibly defined password")
                .is_some()
    });

    let has_clicked = use_state(|| false);

    let sign_in = {
        let has_clicked = has_clicked.clone();
        move |_| has_clicked.set(true)
    };

    let user_or_sign_in = move || -> Html {
        if *has_sign_in {
            html!() // TODO: Create member profile
        } else {
            html! {
                <div class="nav--sign-up" onclick={sign_in}>
                    <h2 class="nav--sign-up-text"> { "SIGN IN" }</h2>
                </div>
            }
        }
    };

    let popup = move || -> Html {
        if *has_clicked {
            html! {
                <SignInPopup />
            }
        } else {
            html!()
        }
    };

    html! {
        <>
            <nav class="nav">
                <a href="/">
                    <img src="/data/OSIS.png" class="nav--img" />
                </a>
                <a href="#">
                    <h2 class="nav--item">{ "BLOG" }</h2>
                </a>
                <a href="http://localhost/about/0">
                    <h2 class="nav--item">{ "ABOUT" }</h2>
                </a>
                <a href="http://localhost/member">
                    <h2 class="nav--item">{ "MEMBERS" }</h2>
                </a>
                <a href="#">
                    <h2 class="nav--item">{ "PROGRAMS" }</h2>
                </a>
                <a href="#">
                    <h2 class="nav--item">{ "EVENTS" }</h2>
                </a>
                { user_or_sign_in() }
            </nav>
            { popup() }
        </>
    }
}

#[function_component(SignInPopup)]
fn sign_in_popup() -> Html {
    let username = use_state(|| None);
    let password = use_state(|| None);
    let response = use_state(|| None);

    let set_username = {
        let username = username.clone();
        move |event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| username.set(Some(input.value())));
        }
    };

    let set_password = {
        let password = password.clone();
        move |event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| password.set(Some(input.value())));
        }
    };

    let onclick = {
        let response = response.clone();
        Callback::from(move |_| {
            let username = username.clone();
            let password = password.clone();
            let response = response.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let sign_in_response = send_sign_in_creds(SignInData {
                    username: username.as_deref().unwrap().to_string(),
                    password: password.as_deref().unwrap().to_string(),
                })
                .await;

                response.set(Some(sign_in_response));
            })
        })
    };

    {
        let response = response.clone();
        use_effect(move || {
            if let Some(SignInResponse(Ok(member))) = &*response {
                let window = window().expect("Unable to get window");
                let storage = window
                    .session_storage()
                    .unwrap()
                    .expect("Unable to get the session storage");

                storage
                    .set_item("username", &member.username)
                    .expect("Unable to set username");
                storage
                    .set_item("password", &member.password)
                    .expect("Unable to set password");

                window.location().reload().expect("Unable to reload page")
            }
            || ()
        })
    };

    let error_message = move || -> Html {
        if let Some(SignInResponse(Err(err))) = &*response {
            html! {
                <div class="sign-in-popup--error">
                    <p class="sign-in-popup--error-text">{ err }</p>
                </div>
            }
        } else {
            html!()
        }
    };

    html! {
        <>
            <div class="dim-page"/>
            <div class="sign-in-popup">
                <h1 class="sign-in-popup--title">{ "Please sign in" }</h1>
                <form class="sign-in-popup--form">
                    { error_message() }
                    <label class="sign-in-popup--label" for="username">{ "Username" }</label>
                    <input class="sign-in-popup--input" type="text" id="username" name="username" onchange={set_username} />
                    <label class="sign-in-popup--label" for="password">{ "Password" }</label>
                    <input class="sign-in-popup--input" type="text" id="password" name="password" onchange={set_password} />
                    <input class="sign-in-popup--button" type="button" value="Sign up" {onclick}/>
                </form>
            </div>
        </>
    }
}

#[function_component(SignUpButton)]
pub fn sign_up_button(props: &SignUpButtonProps) -> Html {
    html! {
        <div class={format!("{} {}", props.class.clone(), props.modifiers.clone())}>
            <p class="sign-up-button--text">{ "Sign up today" }</p>
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
    #[at("/sign_up")]
    SignUp,
}
