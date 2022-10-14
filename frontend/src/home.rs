// Base color is #dd6c22
// Accent is #1f7be0
// Background is white

use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Banner />
            <WhyOsis />
            <Testimonies />
            <WhatIsOsis />
            <Faq />
            <SignUpToday />
        </div>
    }
}

#[function_component(Banner)]
fn banner() -> Html {
    html! {
        <div class="banner cover-image center-vert center-horz-flex center-text height-75 flex">
            <div class="grid center-horz row-gap-med">
                <BannerText />
                <SignUpButton />
            </div>
        </div>
    }
}

#[function_component(BannerText)]
fn banner_text() -> Html {
    html! {
        <div class="flex list-vert margin-base banner-text">
            <h1 class="white font-large">
                <span>{"Empowerment"}</span>
                {", "}
                <span>{"team work"}</span>
                {", and "}
                <span>{"experience"}</span>
            </h1>
            <h2 class="white font-medium">
                { "Make you more " }
                <span class="font-medium">{ "confident" }</span>
                {", give you "}
                <span class="font-medium">{ "control" }</span>
                {". And your "}
                <span class="font-medium">{ "friends" } </span>
                { " will help you along the way" }
            </h2>
        </div>
    }
}

#[function_component(SignUpButton)]
fn sign_up_button() -> Html {
    html! {
        <div class="hover back-white width-15 flex center-vert center-horz">
            <h1 class="font-medium width-100">{ "Sign up for a better future" }</h1>
        </div>
    }
}

#[function_component(WhyOsis)]
fn why_osis() -> Html {
    html! {
        <div class="margin-large margin-hor-0">
            <h1 class="font-xl center-text">{ "Why should I join OSIS?" }</h1>
            <div class="flex wrap center-horz list-vert gap-150 center-vert">
                <WhyOsisSection image_path="data/banner.jpeg" header="A brighter future" align={WhyOsisAlign::Left} color="d98126" link="/about/0">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Give you control" align={WhyOsisAlign::Right} color="267ed9" link="/about/1">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Dream big with team work" align={WhyOsisAlign::Left} color="d98126" link="/about/2">
                    <p class="font-medium">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
            </div>
        </div>
    }
}

#[function_component(WhyOsisSection)]
fn why_osis_section(props: &WhyOsisSectionProp) -> Html {
    let alignment = match props.align {
        WhyOsisAlign::Left => "align-right ver-split-2-1",
        WhyOsisAlign::Right => "align-left ver-split-1-2",
    };

    html! {
        <div class={format!("grid {} center-vert why-osis-section", alignment)}>
            <h1 class="font-large center-text">{ props.header.clone() }</h1>
            <img src={props.image_path.clone()} />
            <div class="area-text flex list-vert">
                <div>
                    { for props.children.iter() }
                </div>
                <div class="info-button">
                    <LearnMoreButton link={props.link.clone()}/>
                </div>
            </div>
        </div>
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
        <div class="margin-base">
            <h1 class="font-xl center-text">{ "Our proof" }</h1>
            <div class="flex wrap space-around gap-50">
                <Testimony header="Lorem ipsum" image_path="data/person.png">
                    <p class="font-medium par">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }</p>
                </Testimony>
                <Testimony header="Lorem ipsum" image_path="data/person.png">
                    <p class="font-medium par">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }</p>
                </Testimony>
                <Testimony header="Lorem ipsum" image_path="data/person.png">
                    <p class="font-medium par">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }</p>
                </Testimony>
            </div>
        </div>
    }
}

#[function_component(Testimony)]
fn testimony(props: &TestimonyProp) -> Html {
    html! {
        <div class="grid width-1-5 min-width-400 center-vert ver-split-1-3 testimony">
            <h1 class="font-large center-text">{ props.header.clone() }</h1>
            <img src={ props.image_path.clone() } class="" />
            <div class="area-text">
                { for props.children.iter() }
            </div>
        </div>
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

// #[function_component(ProgramListMain)]
// fn program_list_main() -> Html {
//     html! {
//         <div class="margin-base">
//             <h1 class="font-xl center-text">{ "Our best programs" }</h1>
//             <div class="flex space-around wrap">
//                 <ProgramEvent />
//                 <ProgramEvent />
//                 <ProgramEvent />
//                 <ProgramEvent />
//             </div>
//         </div>
//     }
// }

// // enum ProgramEventType {
// //     Program,
// //     Event,
// // }
// //
// // #[derive(Properties, PartialEq)]
// // struct ProgramEventProp {
// //     pe_type: ProgramEventType
// // }

// #[function_component(ProgramEvent)]
// fn program_events() -> Html {
//     html! {
//         <div class="grid hor-split-aaa width-1-5 min-width-200 center-vert">
//             <div>
//                 <h1 class="font-large center-text">{ "Lorem ipsum" }</h1>
//             </div>
//             <div class="height-100">
//                 <img src="data/program.jpeg" class="height-100" />
//             </div>
//             <div class="height-100">
//                 <Rating />
//             </div>
//         </div>
//     }
// }

// // #[derive(Properties)]
// // struct RatingProp {
// //     rating: u8
// // }

// #[function_component(Rating)]
// fn rating() -> Html {
//     html! {
//         <div class="grid ver-split-5x1">
//             <img class="height-100" src="data/star.png" />
//             <img class="height-100" src="data/star.png" />
//             <img class="height-100" src="data/star.png" />
//             <img class="height-100" src="data/star.png" />
//             <img class="height-100" src="data/star.png" />
//         </div>
//     }
// }

#[function_component(Faq)]
fn faq() -> Html {
    html! {
        <div>
            <h1 class="font-xl center-text">{ "FAQ" }</h1>
            <div class="flex list-vert margin-base gap-50">
                <FaqSection question="lorem ipsum">
                    {"
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                    Libero justo laoreet sit amet. Ullamcorper dignissim cras tincidunt lobortis feugiat vivamus at. Amet nisl purus in mollis nunc sed id.
                    "}
                </FaqSection>
                <FaqSection question="lorem ipsum">
                    {"
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                    Libero justo laoreet sit amet. Ullamcorper dignissim cras tincidunt lobortis feugiat vivamus at. Amet nisl purus in mollis nunc sed id.
                    "}
                </FaqSection>
                <FaqSection question="lorem ipsum">
                    {"
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                    Libero justo laoreet sit amet. Ullamcorper dignissim cras tincidunt lobortis feugiat vivamus at. Amet nisl purus in mollis nunc sed id.
                    "}
                </FaqSection>
            </div>
        </div>
    }
}

#[function_component(FaqSection)]
fn faq_section(props: &FaqSectionProp) -> Html {
    let dropdown = use_state(|| Dropdown::Hidden);

    let dropdown_cl = dropdown.clone();

    let header = move || match *dropdown_cl {
        Dropdown::Hidden => "+ ",
        Dropdown::Expanded => "- ",
    };

    let dropdown_cl = dropdown.clone();

    let paragraph = move || match *dropdown_cl {
        Dropdown::Hidden => html! {
            ""
        },
        Dropdown::Expanded => html! {
            <p class="center-text font-medium">{ for props.children.iter() }</p>
        },
    };

    let dropdown = dropdown.clone();

    let onclick = {
        let dropdown = dropdown.clone();
        Callback::from(move |_| dropdown.set((*dropdown).switch()))
    };

    html! {
        <div class="flex center-vert list-vert">
            <h1 class="center-text font-large width-fit align-start hover" {onclick}>{ header() }{ props.question.clone() }</h1>
            { paragraph() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FaqSectionProp {
    question: String,
    children: Children,
}

#[derive(Copy, Clone)]
enum Dropdown {
    Expanded,
    Hidden,
}

impl Dropdown {
    fn switch(&self) -> Self {
        match self {
            Dropdown::Expanded => Dropdown::Hidden,
            Dropdown::Hidden => Dropdown::Expanded,
        }
    }
}

#[function_component(SignUpToday)]
fn sign_up_today() -> Html {
    html! {
        <div class="flex center-vert center-horz list-vert">
            <h1 class="font-xl">{ "Sign up for a better future" }</h1>
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
            <form class="margin-base white grid back-base center-horz center-text height-20 width-50">
                <label for="usern" class="font-medium">{ "Username" }</label>
                <textarea type="text" id="usern" name="usern" class="width-50 font-medium" onchange={set_name}/>
                <label for="reason" class="font-medium">{ "Password" }</label>
                <textarea type="text" id="reason" name="reason" class="width-50 font-medium" onchange={set_reason}/><div class="height-50px"/>
                <div class="sign-up-button flex center-horz-flex width-100">
                    <SignUpButton />
                </div>
                <div class="height-50px"/>
            </form>
        }
    }
}

enum SignUpMsg {
    SetUsername(String),
    SetPassword(String),
    // Submit; todo
}
