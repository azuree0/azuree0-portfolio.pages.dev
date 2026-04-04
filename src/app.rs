use crate::api::github;
use crate::components::{Hero, MarineSnow, RepoGrid};
use crate::scene::Scene3d;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use yew::prelude::*;

const EMAIL: &str = "azure.ad@yahoo.com";

/// Root App component: WebGL scene, hero, repo grid, GitHub fetch, and copy-email.
#[function_component(App)]
pub fn app() -> Html {
    // Cached list or static fallback for instant paint; fetch always refreshes from GitHub next.
    let repos = use_state(|| github::initial_repos());
    let tagline_hovered = use_state(|| false);

    let fetch_repos = Rc::new({
        let repos = repos.clone();
        move || {
            let repos = repos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let data = github::fetch_repos_with_fallback().await;
                repos.set(data);
            });
        }
    });

    // Initial fetch (runs in background; fallback already shown)
    {
        let fetch_repos = fetch_repos.clone();
        use_effect_with((), move |_| {
            fetch_repos();
            || ()
        });
    }

    // Tab focus refresh
    {
        let fetch_repos = fetch_repos.clone();
        use_effect_with((), move |_| {
            let document = window().and_then(|w| w.document()).expect("no document");
            let doc_clone = document.clone();
            let listener = EventListener::new(&document, "visibilitychange", move |_| {
                if doc_clone.visibility_state() == web_sys::VisibilityState::Visible {
                    fetch_repos();
                }
            });
            move || drop(listener)
        });
    }

    // Periodic refresh so new public repos appear without a full reload (GitHub unauthenticated limit: 60 req/hr per IP).
    {
        let fetch_repos = fetch_repos.clone();
        use_effect_with((), move |_| {
            let fetch = fetch_repos.clone();
            let _interval = Interval::new(5 * 60 * 1000, move || {
                fetch();
            });
            || ()
        });
    }

    let copy_email = Callback::from(move |_: MouseEvent| {
        if let Some(w) = window() {
            let clipboard = w.navigator().clipboard();
            let email = EMAIL.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                let promise = clipboard.write_text(&email);
                let _ = JsFuture::from(promise).await;
            });
        }
    });

    html! {
        <>
            <Scene3d />
            <div class="overlay">
                <div id="caustics-container" class="caustics-container"></div>
                <MarineSnow />
                <div class="overlay-body">
                    <Hero
                        on_tagline_enter={Callback::from({
                            let tagline_hovered = tagline_hovered.clone();
                            move |_| tagline_hovered.set(true)
                        })}
                        on_tagline_leave={Callback::from({
                            let tagline_hovered = tagline_hovered.clone();
                            move |_| tagline_hovered.set(false)
                        })}
                        on_copy_email={copy_email}
                        email={EMAIL.to_string()}
                    />
                    <main class="content">
                        <RepoGrid repos={(*repos).clone()} show_icosahedron={*tagline_hovered} />
                    </main>
                </div>
            </div>
        </>
    }
}
