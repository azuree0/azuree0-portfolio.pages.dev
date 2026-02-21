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

#[function_component(App)]
pub fn app() -> Html {
    // Start with fallback so content shows immediately; fetch updates in background
    let repos = use_state(|| github::static_fallback());
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

    // Periodic refresh every 30 min
    {
        let fetch_repos = fetch_repos.clone();
        use_effect_with((), move |_| {
            let fetch = fetch_repos.clone();
            let _interval = Interval::new(30 * 60 * 1000, move || {
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
                    />
                    <main class="content">
                        <RepoGrid repos={(*repos).clone()} show_poem={*tagline_hovered} />
                    </main>
                    <footer class="overlay-footer">
                        <button type="button" class="hero-email" onclick={copy_email} title="Copy email">
                            {EMAIL}
                        </button>
                    </footer>
                </div>
            </div>
        </>
    }
}
