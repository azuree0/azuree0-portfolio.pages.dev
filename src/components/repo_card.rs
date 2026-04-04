use crate::models::Repo;
use crate::prefetch::prefetch_navigation_url;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepoCardProps {
    pub repo: Repo,
    #[prop_or(false)]
    pub show_icosahedron: bool,
    /// First cards load images eagerly for LCP; rest use native lazy loading.
    #[prop_or(true)]
    pub image_lazy: bool,
}

/// Single repo card: screenshot (or header/desc), optional icosahedron overlay (tagline hover), link to GitHub.
#[function_component(RepoCard)]
pub fn repo_card(props: &RepoCardProps) -> Html {
    let prefetch_href = props.repo.html_url.clone();
    let on_mouse_enter = Callback::from(move |_| {
        prefetch_navigation_url(&prefetch_href);
    });

    let lang_color = match props.repo.language.as_deref() {
        Some("Rust") => "var(--accent-cyan)",
        Some("C++") => "var(--accent-aqua)",
        Some("Python") => "var(--accent-green)",
        Some("Liquid") => "var(--accent-light)",
        _ => "var(--text-muted)",
    };

    let screenshot_block = props.repo.screenshot.as_ref().map(|img| {
        let wrap_class = if props.show_icosahedron {
            "repo-screenshot-wrap icosahedron-visible"
        } else {
            "repo-screenshot-wrap"
        };
        let loading = if props.image_lazy { "lazy" } else { "eager" };
        html! {
            <div class={wrap_class}>
                if props.show_icosahedron {
                    <div class="icosahedron-overlay" aria-hidden="true">
                        <div class="icosahedron-overlay-host"></div>
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
