use crate::utilities::*;
use rand::random;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct SignUpForm {
    email: String,
    status: SubmissionStatus,
    code: Option<u32>,
    code_entered: Option<u32>,
}

impl SignUpForm {
    fn view_get_email(&self, ctx: &Context<Self>) -> Html {
        let set_email = ctx.link().batch_callback(move |event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| SignUpFormMsg::SetEmail(input.value()))
        });

        let submit = ctx.link().callback(|_| SignUpFormMsg::Submit);

        html! {
            <>
                <NavBar />
                <form class="sign-up-form" action="javascript: void 0">
                    <div class="sign-up-form--form">
                        <div class="sign-up-form--info">
                            <label for="email" class="sign-up-form--field-name">{ "Email" }</label>
                            <input type="text" id="email" name="email" class="sign-up-form--field-input"
                                onchange={set_email}/>
                        </div>
                        <div class="sign-up-form--sign-up-button">
                            <input type="submit" value="Sign up" onclick={submit} class="sign-up-today sign-up-today--hover"/>
                        </div>
                    </div>
                </form>
            </>
        }
    }

    fn view_enter_code(&self, ctx: &Context<Self>) -> Html {
        let code = self.code;
        let code_entered = self.code_entered;

        log::info!("code = {}", code.unwrap());

        let set_code = ctx.link().batch_callback(move |event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let input: Option<u32> =
                input.map(|input| input.value().parse().expect("Please enter a number")); // TODO: Better error handling

            input.map(SignUpFormMsg::SetCode)
        });

        let on_submit = move |_| {
            if code_entered == code {
                log::info!("Code matched")
            } else {
                log::info!("Code does not match")
            };
        };

        html! {
            <div class="enter-code">
                <h1 class="enter-code--title">{ "Please enter the code we have sent via gmail" }</h1>
                <h2 class="enter-code--subtitle">{ format!("We have sent you a code through your email address {}. Please enter the code we gave you to continue your sign up", self.email) }</h2>
                <form action="javascript: void 0" class="enter-code--form">
                    <label class="enter-code--form-title" for="code">{ "Code" }</label>
                    <input class="enter-code--input" type="text" id="code" name="code" onchange={set_code} />
                    <input class="enter-code--enter" type="submit" value="Enter" onclick={on_submit} />
                </form>
            </div>
        }
    }
}

impl Component for SignUpForm {
    type Message = SignUpFormMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            email: "".to_string(),
            status: SubmissionStatus::NotSubmitted,
            code: None,
            code_entered: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignUpFormMsg::SetEmail(x) => {
                self.email = x;
                true
            }
            SignUpFormMsg::SetCode(code) => {
                self.code_entered = Some(code);
                true
            }
            SignUpFormMsg::Submit => {
                self.status = SubmissionStatus::Submitted;
                self.code = Some(random::<u32>());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.status {
            SubmissionStatus::Submitted => self.view_enter_code(ctx),
            SubmissionStatus::NotSubmitted => self.view_get_email(ctx),
        }
    }
}

enum SubmissionStatus {
    Submitted,
    NotSubmitted,
}

pub enum SignUpFormMsg {
    SetEmail(String),
    SetCode(u32),
    Submit,
}
