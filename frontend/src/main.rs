mod about;
mod home;
mod landing;
mod member;

use about::About;
use member::MemberComp;
use home::Home;
use landing::Landing;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about/:id")]
    About { id: u8 },
    #[at("/landing")]
    Landing,
    #[at("/member/:id")]
    Member { id: u32 },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <Home />
        },
        Route::About { id } => html! {
            <About content_id={ *id }/>
        },
        Route::Landing => html! {
            <Landing />
        },
        Route::Member { id } => html! {
            <MemberComp member_id={ *id } />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    )
}

fn main() {
    yew::start_app::<App>();
}
