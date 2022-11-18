use crate::utilities::*;
use yew::prelude::*;

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
    let alignment = match props.align {
        // alignment of text
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
            <SignUpButton class="sign-up-button-today sign-up-button--hover"/>
        </div>
    }
}
