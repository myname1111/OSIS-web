mod about;
mod home;

use about::About;
use home::Home;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about/:id")]
    About { id: u8 },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <Home />
        },
        Route::About { id } => html! {
            <About content_id={ *id }/>
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
