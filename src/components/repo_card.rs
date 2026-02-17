use crate::models::Repo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepoCardProps {
    pub repo: Repo,
}

#[function_component(RepoCard)]
pub fn repo_card(props: &RepoCardProps) -> Html {
    let lang_color = match props.repo.language.as_deref() {
        Some("Rust") => "var(--accent-cyan)",
        Some("C++") => "var(--accent-aqua)",
        Some("Python") => "var(--accent-green)",
        Some("Liquid") => "var(--accent-light)",
        _ => "var(--text-muted)",
    };

    let screenshot_block = props.repo.screenshot.as_ref().map(|img| {
        html! {
            <div class="repo-screenshot-wrap">
                <img src={img.clone()} alt={props.repo.name.clone()} class="repo-screenshot-full" />
            </div>
        }
    });

    html! {
        <a
            href={props.repo.html_url.clone()}
            target="_blank"
            rel="noopener noreferrer"
            class="repo-card"
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
