use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let todo = String::from("Todo ").repeat(100);
    html! {
        { todo }
    }
}
