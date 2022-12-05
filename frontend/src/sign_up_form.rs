use crate::utilities::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub struct SignUpForm {
    email: String,
    status: SubmissionStatus,
}

enum SubmissionStatus {
    Submitted,
    NotSubmitted,
}

impl Component for SignUpForm {
    type Message = SignUpFormMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            email: "".to_string(),
            status: SubmissionStatus::NotSubmitted,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignUpFormMsg::SetEmail(x) => {
                self.email = x;
                true
            }
            SignUpFormMsg::Submit => {
                self.status = SubmissionStatus::Submitted;
                log::info!("Email is {}", self.email); // TODO: Turn this into logic

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_email = ctx.link().batch_callback(move |event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            input.map(|input| SignUpFormMsg::SetEmail(input.value()))
        });

        let submit = ctx.link().callback(|_| SignUpFormMsg::Submit);

        html! {
            <>
                <NavBar />
                <form class="sign-up-form">
                    <div class="sign-up-form--form">
                        <div class="sign-up-form--info">
                            <label for="usern" class="sign-up-form--field-name">{ "Email" }</label>
                            <input type="text" id="usern" name="usern" class="sign-up-form--field-input"
                                onchange={set_email}/>
                        </div>
                        <div class="sign-up-form--sign-up-button-container">
                            <input type="submit" value="Sign up" onclick={submit}/>
                        </div>
                    </div>
                </form>
            </>
        }
    }
}

pub enum SignUpFormMsg {
    SetEmail(String),
    Submit,
}
