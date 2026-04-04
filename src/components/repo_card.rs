use crate::models::Repo;
use crate::prefetch::prefetch_navigation_url;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepoCardProps {
    pub repo: Repo,
    #[prop_or(false)]
    pub show_poem: bool,
    /// First cards load images eagerly for LCP; rest use native lazy loading.
    #[prop_or(true)]
    pub image_lazy: bool,
}

/// Calls global `portfolioInitIcosahedron` (Three.js) when the tagline overlay is shown.
fn js_init_icosahedron() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(js_val) = js_sys::Reflect::get(&window, &JsValue::from_str("portfolioInitIcosahedron")) else {
        return;
    };
    if js_val.is_undefined() || js_val.is_null() {
        return;
    }
    let Ok(f) = js_val.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = f.call0(&JsValue::NULL);
}

/// Calls global `portfolioDisposeIcosahedron` to tear down WebGL and RAF.
fn js_dispose_icosahedron() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(js_val) = js_sys::Reflect::get(&window, &JsValue::from_str("portfolioDisposeIcosahedron")) else {
        return;
    };
    if js_val.is_undefined() || js_val.is_null() {
        return;
    }
    let Ok(f) = js_val.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = f.call0(&JsValue::NULL);
}

/// Single repo card: screenshot (or header/desc), optional 3D icosahedron overlay on hover, link to GitHub.
#[function_component(RepoCard)]
pub fn repo_card(props: &RepoCardProps) -> Html {
    let prefetch_href = props.repo.html_url.clone();
    let on_mouse_enter = Callback::from(move |_| {
        prefetch_navigation_url(&prefetch_href);
    });

    {
        let show_poem = props.show_poem;
        use_effect_with(show_poem, move |show_poem| {
            let pending = if *show_poem {
                Some(Timeout::new(0, move || {
                    js_init_icosahedron();
                }))
            } else {
                None
            };
            move || {
                drop(pending);
                js_dispose_icosahedron();
            }
        });
    }

    let lang_color = match props.repo.language.as_deref() {
        Some("Rust") => "var(--accent-cyan)",
        Some("C++") => "var(--accent-aqua)",
        Some("Python") => "var(--accent-green)",
        Some("Liquid") => "var(--accent-light)",
        _ => "var(--text-muted)",
    };

    let screenshot_block = props.repo.screenshot.as_ref().map(|img| {
        let wrap_class = if props.show_poem {
            "repo-screenshot-wrap poem-visible"
        } else {
            "repo-screenshot-wrap"
        };
        let loading = if props.image_lazy { "lazy" } else { "eager" };
        html! {
            <div class={wrap_class}>
                if props.show_poem {
                    <div class="repo-icosahedron-overlay" aria-hidden="true">
                        <div id="first-repo-icosahedron-host" class="repo-icosahedron-host"></div>
                    </div>
                }
                <img
                    src={img.clone()}
                    alt={props.repo.name.clone()}
                    class="repo-screenshot-full"
                    loading={loading}
                    decoding="async"
                />
            </div>
        }
    });

    html! {
        <a
            href={props.repo.html_url.clone()}
            target="_blank"
            rel="noopener noreferrer"
            class="repo-card"
            onmouseenter={on_mouse_enter.clone()}
        >
            if let Some(block) = screenshot_block {
                {block}
            } else {
                <div class="repo-card-header">
                    <h3 class="repo-name">{&props.repo.name}</h3>
                    if let Some(ref lang) = props.repo.language {
                        <span class="repo-lang" style={format!("--lang-color: {}", lang_color)}>
                            {lang}
                        </span>
                    }
                </div>
                if let Some(ref desc) = props.repo.description {
                    <p class="repo-desc">{desc}</p>
                }
                <div class="repo-meta">
                    if props.repo.stargazers_count > 0 {
                        <span>{"★ "}{props.repo.stargazers_count}</span>
                    }
                </div>
            }
        </a>
    }
}
