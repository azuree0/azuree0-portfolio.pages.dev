use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub on_tagline_enter: Callback<MouseEvent>,
    pub on_tagline_leave: Callback<MouseEvent>,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <header class="hero">
            <h1 class="hero-title">{"Azure"}</h1>
            <span
                class="hero-tagline-wrap"
                onmouseenter={props.on_tagline_enter.clone()}
                onmouseleave={props.on_tagline_leave.clone()}
            >
                <p class="hero-tagline">{"Freelance code, Portfolio"}</p>
            </span>
        </header>
    }
}
