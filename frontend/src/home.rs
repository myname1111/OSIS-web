use yew::prelude::*;
use crate::utilities::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <NavBar />
            <Banner />
            <WhyOsis />
            <WhatIsOsis />
            <Faq />
            <ProgramList />
            <EventList />
            <ContactList />
        </>
    }
}

#[function_component(Banner)]
fn banner() -> Html {
    html! {
        <div class="banner banner--home">
            <div class="banner--container">
                <BannerText />
                <SignUpButton modifiers="sign-up-button--banner"/>
            </div>
        </div>
    }
}

#[function_component(BannerText)]
fn banner_text() -> Html {
    html! {
        <div class="banner-home--text-container">
            <h1 class="banner--text">
                {"Empowerment, team work, and experience"}
            </h1>
        </div>
    }
}

#[function_component(WhyOsis)]
fn why_osis() -> Html {
    html! {
        <section class="why-osis">
            <h1 class="header">{ "Why osis" }</h1>
            <div class="why-osis--container why-osis--container--home">
                <WhyOsisSection image_path="data/banner.jpeg" header="Experience" link="/about/0">
                    { "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Collaboration" link="/about/1">
                    { "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}
                </WhyOsisSection>
                <WhyOsisSection image_path="data/banner.jpeg" header="Empowerment" link="/about/2">
                    { "
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                        Eros donec ac odio tempor orci dapibus ultrices.
                    "}
                </WhyOsisSection>
            </div>
        </section>
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
        <article class="why-osis-section-home">
            <h1 class="why-osis-section-home--header">{ props.header.clone() }</h1>
            <img src={props.image_path.clone()} />
            <p class="why-osis-section-home--par">{ for props.children.iter() }</p>
            <LearnMoreButton link={ props.link.clone() }/>
        </article>
    }
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
            <img src="data/banner.jpeg" class="what-osis--img"/>
            <p class="what-osis--par">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
            Ipsum suspendisse ultrices gravida dictum fusce ut. Nibh tellus molestie nunc non blandit massa enim nec. In arcu cursus euismod quis viverra." }</p>
            <LearnMoreButton link="/about/3"/>
        </article>
    }
}

#[derive(Properties, PartialEq)]
struct LearnMoreButtonProp {
    link: String,
}

#[function_component(LearnMoreButton)]
fn learn_more_button(props: &LearnMoreButtonProp) -> Html {
    html! {
        <a href={props.link.clone()} class="learn-more-button">
            <h1 class="learn-more-button--text">{ "Learn more" }</h1>
        </a>
    }
}

#[function_component(Faq)]
fn faq() -> Html {
    html! {
        <section class="faq">
            <h1 class="header">{ "FAQ" }</h1>
            <div class="faq--container">
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
        </section>
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
            <p class="faq-section--content">{ for props.children.iter() }</p>
        },
    };


    let onclick = {
        let dropdown = dropdown;
        Callback::from(move |_| dropdown.set((*dropdown).switch()))
    };

    html! {
        <div class="faq-section">
            <h1 class="faq-section--question faq-section--hover" {onclick}>{ header() }{ props.question.clone() }</h1>
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

#[function_component(ProgramList)]
fn program_list() -> Html {
    html! {
        <section class="item-list">
            <h1 class="header">{ "See our best programs" }</h1>
            <div class="item-list--section">
                <Program />
                <Program />
                <Program />
                <Program />
            </div>
            <a href="/todo/program"><h2 class="item-list--link">{ "Click here to see more programs" }</h2></a>
        </section>
    }
}

#[function_component(EventList)]
fn event_list() -> Html {
    html! {
        <section class="item-list">
            <h1 class="item-list--header">{ "See our latest events" }</h1>
            <div class="item-list--section">
                <Event />
                <Event />
                <Event />
                <Event />
            </div>
            <a href="/todo/event"><h2 class="item-list--link">{ "Click here to see more events" }</h2></a>
        </section>
    }
}


#[function_component(Event)]
fn event() -> Html {
    html! {
        <article class="item">
            <div>
                <h1 class="item--title">{ "Lorem ipsum" }</h1>
            </div>
            <img src="data/program.jpeg" class="item--image" />
            <div class="rating--container">
                <Rating />
            </div>
        </article>
    }
}

#[function_component(Program)]
fn program() -> Html {
    html! {
        <article class="item">
            <div>
                <h1 class="item--title">{ "Lorem ipsum" }</h1>
            </div>
            <img src="data/program.jpeg" class="item--image" />
            <div class="rating--container">
                <Rating />
            </div>
        </article>
    }
}
// #[derive(Properties)]
// struct RatingProp {
//     rating: u8
// }

#[function_component(Rating)]
fn rating() -> Html {
    html! {
        <div class="rating">
            { (0..6)
                .into_iter()
                .map(|_| html!{ <img src="data/star.png" class="rating--img"/> })
                .collect::<Html>() }
        </div>
    }
}

#[function_component(ContactList)]
fn contact_list() -> Html {
    html! {
        <div class="contact">
            <h2 class="header">{ "Contacts" }</h2>
            <div class="contact--list">
                <Contact image="data/whatsapp.webp" name="XXXX-XXXX-XXXX" />
                <Contact image="data/whatsapp.webp" name="XXXX-XXXX-XXXX" />
                <Contact image="data/whatsapp.webp" name="XXXX-XXXX-XXXX" />
                <Contact image="data/whatsapp.webp" name="XXXX-XXXX-XXXX" />
            </div>
        </div>
    }
}

#[function_component(Contact)]
fn contact(props: &ContactProp) -> Html {
    html! {
        <div class="contact--item">
            <img src={ props.image.clone() } class="contact--img"/>
            <h2>
                { props.name.clone() }
            </h2>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ContactProp {
    image: String,
    name: String,
}
