// Base color is #dd6c22
// Accent is #1f7be0
// Background is white

use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use crate::utilities::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div>
            <Banner />
            <WhyOsis />
            <Testimonies />
            <WhatIsOsis />
            <SignUpToday />
        </div>
    }
}

#[function_component(Banner)]
fn banner() -> Html {
    html! {
        <header class="banner banner--landing">
            <div class="banner--container">
                <BannerText />
                <SignUpButton modifiers="sign-up-button--banner"/>
            </div>
        </header>
    }
}

#[function_component(BannerText)]
fn banner_text() -> Html {
    html! {
        <div class="banner-landing--text-container">
            <h1 class="banner--text">
                {"Empowerment, team work, and experience"}
            </h1>
            <h2 class="banner-landing--sub">
                {"Make you more confident, give you control. And your friends will help you along the way"}
            </h2>
        </div>
    }
}

#[function_component(WhyOsis)]
fn why_osis() -> Html {
    html! {
        <section class="why-osis why-osis--landing">
            <h2 class="header">{ "Why should I join OSIS?" }</h2>
            <div class="why-osis-container-landing">
                <WhyOsisSection image_path="data/banner.jpeg" header="A brighter future" align={WhyOsisAlign::Left} color="d98126" link="/about/0">
                    { "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Give you control" align={WhyOsisAlign::Right} color="267ed9" link="/about/1">
                    { "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Dream big with team work" align={WhyOsisAlign::Left} color="d98126" link="/about/2">
                    { "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}
                </WhyOsisSection>
            </div>
        </section>
    }
}

#[function_component(WhyOsisSection)]
fn why_osis_section(props: &WhyOsisSectionProp) -> Html {
    let alignment = match props.align { // alignment of text
        WhyOsisAlign::Left => "why-osis-section-landing--left",
        WhyOsisAlign::Right => "why-osis-section-landing--right",
    };

    html! {
        <article class={format!("why-osis-section-landing {}", alignment)}>
            <h3 class="why-osis-section-landing--title">{ props.header.clone() }</h3>
            <img src={props.image_path.clone()} class="why-osis-section-landing--img"/>
            <div class="why-osis-section-landing--par-container">
                <p class="why-osis-section-landing--par">{ for props.children.iter() }</p>
            </div>
        </article>
    }
}

#[derive(Properties, PartialEq)]
struct WhyOsisSectionProp {
    children: Children,
    header: String,
    image_path: String,
    align: WhyOsisAlign,
    color: String,
    link: String,
}

#[derive(PartialEq)]
enum WhyOsisAlign {
    Left,
    Right,
}

#[function_component(Testimonies)]
fn testimonies() -> Html {
    html! {
        <section class="testimonies">
            <h2 class="header">{ "Our proof" }</h2>
            <div class="testimonies--testimony-list">
                <Testimony header="Lorem ipsum" image_path="data/person.png">
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }
                </Testimony>
                <Testimony header="Lorem ipsum" image_path="data/person.png">
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }
                </Testimony>
                <Testimony header="Lorem ipsum" image_path="data/person.png">
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }
                </Testimony>
            </div>
        </section>
    }
}

#[function_component(Testimony)]
fn testimony(props: &TestimonyProp) -> Html {
    html! {
        <article class="testimony">
            <h3 class="testimony--from">{ props.header.clone() }</h3>
            <img src={ props.image_path.clone() } class="testimony--img" />
            <div class="testimony--text-container">
                <p class="testimony--par">{ for props.children.iter() }</p>
            </div>
        </article>
    }
}

#[derive(Properties, PartialEq)]
struct TestimonyProp {
    header: String,
    children: Children,
    image_path: String,
}

#[function_component(WhatIsOsis)]
fn what_is_osis() -> Html {
    html! {
        <section class="what-osis">
            <h1 class="header">{ "What is osis" }</h1>
            <OsisInfo />
        </section>
    }
}

#[function_component(OsisInfo)]
fn osis_info() -> Html {
    html! {
        <article class="what-osis--info">
            <img src="data/banner.jpeg" class="img"/>
            <p class="what-osis--par">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
            Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }</p>
        </article>
    }
}

#[function_component(SignUpToday)]
fn sign_up_today() -> Html {
    html! {
        <div class="sign-up-today">
            <h2 class="header">{ "Sign up for a better future" }</h2>
            <SignUp />
        </div>
    }
}

struct SignUp {
    username: String,
    password: String,
}

impl Component for SignUp {
    type Message = SignUpMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignUpMsg::SetUsername(x) => {
                self.username = x;
                true
            }
            SignUpMsg::SetPassword(x) => {
                self.password = x;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_name = ctx.link().batch_callback(|event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            input.map(|input| SignUpMsg::SetUsername(input.value()))
        });

        let set_reason = ctx.link().batch_callback(|event: Event| {
            let target = event.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            input.map(|input| SignUpMsg::SetPassword(input.value()))
        });

        html! {
            <form class="sign-up-form">
                <label for="usern" class="sign-up-form--field-name">{ "Username" }</label>
                <textarea type="text" id="usern" name="usern" class="sign-up-form--field" onchange={set_name}/>
                <label for="reason" class="sign-up-form--field-name">{ "Password" }</label>
                <textarea type="text" id="reason" name="reason" class="sign-up-form--field" onchange={set_reason}/><div class="height-50px"/>
                <div class="sign-up-form--sign-up-button-container">
                    <SignUpButton class="sign-up-button-today sign-up-button--hover"/>
                </div>
            </form>
        }
    }
}

enum SignUpMsg {
    SetUsername(String),
    SetPassword(String),
    // Submit; todo
}
