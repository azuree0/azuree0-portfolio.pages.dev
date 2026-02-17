use crate::components::RepoCard;
use crate::models::Repo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepoGridProps {
    pub repos: Vec<Repo>,
}

#[function_component(RepoGrid)]
pub fn repo_grid(props: &RepoGridProps) -> Html {
    html! {
        <section class="repo-grid">
            {props.repos.iter().enumerate().map(|(i, repo)| {
                let is_first = i == 0;
                html! {
                    <div
                        key={repo.name.clone()}
                        class={if is_first { "scroll-reveal scroll-reveal--first" } else { "scroll-reveal" }}
                    >
                        <RepoCard repo={repo.clone()} />
                    </div>
                }
            }).collect::<Html>()}
        </section>
    }
}
