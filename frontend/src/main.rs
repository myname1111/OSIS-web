// use wasm_bindgen::JsCast;
// use web_sys::HtmlTextAreaElement;
mod home;

use home::MainPage;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/faq")]
    Faq,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <MainPage />
        },
        Route::Faq => html! {
            <Redirect<Route> to={Route::Home}/>
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
