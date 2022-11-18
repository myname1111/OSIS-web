mod about;
mod backend;
mod home;
mod landing;
mod member;
mod member_list;
mod sign_up_form;
mod utilities;

use about::About;
use home::Home;
use landing::Landing;
use member::MemberComp;
use member_list::MemberList;
use sign_up_form::SignUpForm;
use utilities::*;

use yew::prelude::*;
use yew_router::prelude::*;

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
        },
        Route::SignUp => html! {
            <SignUpForm />
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
