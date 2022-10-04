// use wasm_bindgen::JsCast;
// use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

fn main() {
    yew::start_app::<MainPage>();
}

#[function_component(MainPage)]
fn main_page() -> Html {
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
        <div class="hover back-white width-15 height-15 flex center-vert center-horz">
            <h1 class="font-medium width-100">{ "Sign up Today" }</h1>
        </div>
    }
}

// struct SignUp {
//     username: String,
//     reason: String,
// }

// impl Component for SignUp {
//     type Message = SignUpMsg;
//     type Properties = ();

//     fn create(_: &Context<Self>) -> Self {
//         Self {
//             username: "".to_string(),
//             reason: "".to_string(),
//         }
//     }

//     fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             SignUpMsg::SetUsername(x) => {
//                 self.username = x;
//                 true
//             }
//             SignUpMsg::SetReason(x) => {
//                 self.reason = x;
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let set_name = ctx.link().batch_callback(|event: Event| {
//             let target = event.target();

//             let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

//             input.map(|input| SignUpMsg::SetUsername(input.value()))
//         });

//         let set_reason = ctx.link().batch_callback(|event: Event| {
//             let target = event.target();

//             let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

//             input.map(|input| SignUpMsg::SetReason(input.value()))
//         });

//         html! {
//             <form class="white grid">
//                 <h1 class="white font-large">{ "Sign up today!" }</h1>
//                 <label for="usern" class="font-medium">{ "Username" }</label><br/>
//                 <textarea type="text" id="usern" name="usern" rows="1" onchange={set_name}/><br/>
//                 <label for="reason" class="font-medium">{ "Reason to join" }</label><br/>
//                 <textarea type="text" id="reason" name="reason" rows="12" onchange={set_reason}/>
//             </form>
//         }
//     }
// }

// enum SignUpMsg {
//     SetUsername(String),
//     SetReason(String),
//     // Submit; todo
// }

#[function_component(WhyOsis)]
fn why_osis() -> Html {
    html! {
        <div class="margin-base">
            <h1 class="font-xl center-text">{ "Why osis" }</h1>
            <div class="flex wrap space-around">
                <WhyOsisSection image_path="data/banner.jpeg" header="Experience">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Collaboration">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Empowerment">
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
}

#[function_component(WhyOsisSection)]
fn why_osis_section(props: &WhyOsisSectionProp) -> Html {
    html! {
        <div class="grid hor-split-1-1-auto width-1-5 min-width-400 center-vert">
            <h1 class="font-large">{ props.header.clone() }</h1>
            <img src={props.image_path.clone()} />
            <div>
                { for props.children.iter() }
                <div id="info-button">
                    <LearnMoreButton />
                </div>
            </div>
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
            <LearnMoreButton />
        </div>
    }
}

#[function_component(LearnMoreButton)]
fn learn_more_button() -> Html {
    html! {
        <div class="hover back-dark-green but center-text margin-smallest width-50">
            <h1 class="font-medium white-blue">{ "Learn more" }</h1>
        </div>
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
        <div class="grid ver-split-5x1">
            <img class="height-100" src="data/star.png" />
            <img class="height-100" src="data/star.png" />
            <img class="height-100" src="data/star.png" />
            <img class="height-100" src="data/star.png" />
            <img class="height-100" src="data/star.png" />
        </div>
    }
}
