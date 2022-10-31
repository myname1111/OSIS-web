mod about;
mod home;
mod landing;
mod member;
mod backend;
mod member_list;

use about::About;
use member::MemberComp;
use home::Home;
use landing::Landing;
use member_list::MemberList;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route { // TODO: create 404
    #[at("/")]
    Home,
    #[at("/about/:id")]
    About { id: u8 },
    #[at("/landing")]
    Landing,
    #[at("/member/:id")]
    Member { id: u32 },
    #[at("/member")]
    MemberList
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
        Route::MemberList => html! {
            <MemberList />
        }
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
