use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub on_tagline_enter: Callback<MouseEvent>,
    pub on_tagline_leave: Callback<MouseEvent>,
    pub on_copy_email: Callback<MouseEvent>,
    pub email: String,
}

/// Hero section: title, tagline, and copy-to-clipboard email button. Tagline hover triggers poem.
#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    let email_hovered = use_state(|| false);

    html! {
        <header class="hero">
            <h1 class="hero-title">{"Azure"}</h1>
            <span class="hero-tagline-wrap">
                <p
                    class="hero-tagline"
                    onmouseenter={props.on_tagline_enter.clone()}
                    onmouseleave={props.on_tagline_leave.clone()}
                >
                    {"Freelance coder, Portfolio"}
                </p>
                <button
                    type="button"
                    class="hero-email"
                    onclick={props.on_copy_email.clone()}
                    onmouseenter={Callback::from({
                        let email_hovered = email_hovered.clone();
                        move |_| email_hovered.set(true)
                    })}
                    onmouseleave={Callback::from({
                        let email_hovered = email_hovered.clone();
                        move |_| email_hovered.set(false)
                    })}
                    title="Copy email"
                >
                    if *email_hovered {
                        <span class="hero-email-copy">{"Copy email"}</span>
                    } else {
                        {&props.email}
                    }
                </button>
            </span>
        </header>
    }
}
