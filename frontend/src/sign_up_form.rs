use crate::utilities::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub struct SignUpForm {
    username: String,
    password: String,
}

impl Component for SignUpForm {
    type Message = SignUpFormMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignUpFormMsg::SetUsername(x) => {
                self.username = x;
                true
            }
            SignUpFormMsg::SetPassword(x) => {
                self.password = x;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_field = |event: Event| {
            let target = event.target();

            target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
        };

        let set_name = ctx.link().batch_callback(move |event: Event| {
            let input = set_field(event);

            input.map(|input| SignUpFormMsg::SetUsername(input.value()))
        });

        let set_reason = ctx.link().batch_callback(move |event: Event| {
            let input = set_field(event);

            input.map(|input| SignUpFormMsg::SetPassword(input.value()))
        });

        html! {
            <>
                <NavBar />
                <form class="sign-up-form">
                    <div class="sign-up-form--form">
                        <div class="sign-up-form--info">
                            <div class="sign-up-form--field">
                                <label for="usern" class="sign-up-form--field-name">{ "Username" }</label>
                                <textarea type="text" id="usern" name="usern" class="sign-up-form--field-input"
                                    onchange={set_name}/>
                            </div>
                            <div class="sign-up-form--field">
                                <label for="reason" class="sign-up-form--field-name">{ "Password" }</label>
                                <textarea type="text" id="reason" name="reason" class="sign-up-form--field-input"
                                    onchange={set_reason}/><div class="height-50px"/>
                            </div>
                        </div>
                        <div class="sign-up-form--sign-up-button-container">
                            <SignUpButton class="sign-up-button-today sign-up-button--hover"/>
                        </div>
                    </div>
                </form>
            </>
        }
    }
}

pub enum SignUpFormMsg {
    SetUsername(String),
    SetPassword(String),
    // Submit; todo
}