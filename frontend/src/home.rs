use yew::prelude::*;
#[function_component(MainPage)]
pub fn main_page() -> Html {
    html! {
        <div>
            <Banner />
            <WhyOsis />
            <Testimonies />
            <WhatIsOsis />
            <Faq />
        </div>
    }
}

#[function_component(Banner)]
fn banner() -> Html {
    html! {
        <div class="banner cover-image center-vert center-horz center-text height-75 flex">
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
                { "Osis will make you more " }
                <span class="font-large">{"confident"}</span>
                {", give you "}
                <span class="font-large">{ "control" } </span>
                {" over school program and events. And the best thing, your "}
                <span class="font-large">{ "friends" } </span>
                { " will help you along the way" }
            </h2>
        </div>
    }
}

#[function_component(SignUpButton)]
fn sign_up_button() -> Html {
    html! {
        <div class="hover back-white width-15 flex center-vert center-horz">
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

// update this with colors
// #d98126
// #267ed9
#[function_component(WhyOsis)]
fn why_osis() -> Html {
    html! {
        <div class="margin-large margin-hor-0">
            <h1 class="font-xl center-text">{ "Why should i join OSIS?" }</h1>
            <div class="flex wrap space-around list-vert gap-250">
                <WhyOsisSection image_path="data/banner.jpeg" header="Get a brighter future" align={WhyOsisAlign::Left} color="d98126">
                    <p class="font-medium white">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Empower you to do more" align={WhyOsisAlign::Right} color="267ed9">
                    <p class="font-medium white">{ "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}</p>
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Team work makes the dream work" align={WhyOsisAlign::Left} color="d98126">
                    <p class="font-medium white">{ "
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

    let style = format!(
        "
        background-color: {};
    ",
        props.color.clone()
    );

    html! {
        <div {style}>
            <div class={format!("grid {} center-vert why-osis-section margin-hor-large", alignment)}>
                <h1 class="font-large center-text white">{ props.header.clone() }</h1>
                <img src={props.image_path.clone()} />
                <div class="area-text flex list-vert">
                    <div>
                        { for props.children.iter() }
                    </div>
                    <div class="info-button">
                        <LearnMoreButton />
                    </div>
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
            <h1 class="font-xl center-text">{ "Testimonies" }</h1>
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
