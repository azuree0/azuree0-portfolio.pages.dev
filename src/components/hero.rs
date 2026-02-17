use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <header class="hero">
            <h1 class="hero-title">{"Azure"}</h1>
            <p class="hero-tagline">{"Freelance code, Portfolio"}</p>
        </header>
    }
}
