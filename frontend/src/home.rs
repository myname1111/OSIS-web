// use wasm_bindgen::JsCast;
// use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Banner />
            <WhyOsis />
            <WhatIsOsis />
            <ProgramListMain />
        </div>
    }
}

#[function_component(Banner)]
fn banner() -> Html {
    html! {
        <div class="banner cover-image center-vert center-horz center-text height-100">
            <div class="grid center-horz height-100 row-gap-med">
                <BannerText />
                <SignUpButton />
            </div>
        </div>
    }
}

#[function_component(BannerText)]
fn banner_text() -> Html {
    html! {
        <div class="height-100 bottom-vert flex">
            <h1 class="white font-large">
                <span>
                    {"Empowerment"}
                </span>
                {", "}
                <span>
                    {"team work"}
                </span>
                {", and "}
                <span>
                    {"experience"}
                </span>
            </h1>
        </div>
    }
}

#[function_component(SignUpButton)]
fn sign_up_button() -> Html {
    html! {
        <div class="hover back-white width-30 height-30 flex center-vert center-horz">
            <h1 class="font-medium width-100">{ "Sign up for a better future" }</h1>
        </div>
    }
}

#[function_component(WhyOsis)]
fn why_osis() -> Html {
    html! {
        <div class="margin-base">
            <h1 class="font-xl center-text">{ "Why osis" }</h1>
            <div class="flex wrap space-around">
                <WhyOsisSection image_path="data/banner.jpeg" header="Experience" link="/about/0">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Collaboration" link="/about/1">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Empowerment" link="/about/2">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct WhyOsisSectionProp {
    children: Children,
    header: String,
    image_path: String,
    link: String,
}

#[function_component(WhyOsisSection)]
fn why_osis_section(props: &WhyOsisSectionProp) -> Html {
    html! {
        <div class="flex list-vert width-20 min-width-400">
            <h1 class="font-large center-text">{ props.header.clone() }</h1>
            <img src={props.image_path.clone()} />
            { for props.children.iter() }
            <LearnMoreButton link={ props.link.clone() }/>
        </div>
    }
}

#[function_component(WhatIsOsis)]
fn what_is_osis() -> Html {
    html! {
        <div>
            <h1 class="font-xl center-text">{ "What is osis" }</h1>
            <OsisInfo />
        </div>
    }
}

#[function_component(OsisInfo)]
fn osis_info() -> Html {
    html! {
        <div class="grid hor-split-3-1 vert-split-3-4 margin-base column-gap-med osis-info">
            <img src="data/banner.jpeg" class="img"/>
            <p class="font-medium par">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
            Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }</p>
            <LearnMoreButton link="/about/3"/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LearnMoreButtonProp {
    link: String,
}

#[function_component(LearnMoreButton)]
fn learn_more_button(props: &LearnMoreButtonProp) -> Html {
    html! {
        <a href={props.link.clone()} class="back-accent but center-text margin-smallest width-50 link-nochange">
            <h1 class="font-medium white">{ "Learn more" }</h1>
        </a>
    }
}

#[function_component(ProgramListMain)]
fn program_list_main() -> Html {
    html! {
        <div class="margin-base">
            <h1 class="font-xl center-text">{ "Our best programs" }</h1>
            <div class="flex space-around wrap">
                <ProgramEvent />
                <ProgramEvent />
                <ProgramEvent />
                <ProgramEvent />
            </div>
        </div>
    }
}

// enum ProgramEventType {
//     Program,
//     Event,
// }
//
// #[derive(Properties, PartialEq)]
// struct ProgramEventProp {
//     pe_type: ProgramEventType
// }

#[function_component(ProgramEvent)]
fn program_events() -> Html {
    html! {
        <div class="grid hor-split-aaa width-1-5 min-width-200 center-vert">
            <div>
                <h1 class="font-large center-text">{ "Lorem ipsum" }</h1>
            </div>
            <div class="height-100">
                <img src="data/program.jpeg" class="height-100" />
            </div>
            <div class="height-100">
                <Rating />
            </div>
        </div>
    }
}

// #[derive(Properties)]
// struct RatingProp {
//     rating: u8
// }

#[function_component(Rating)]
fn rating() -> Html {
    html! {
        <div class="flex rating">
            <img src="data/star.png" />
            <img src="data/star.png" />
            <img src="data/star.png" />
            <img src="data/star.png" />
            <img src="data/star.png" />
        </div>
    }
}
