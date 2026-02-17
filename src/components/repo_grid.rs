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
            {props.repos.iter().map(|repo| {
                html! {
                    <div key={repo.name.clone()} class="scroll-reveal">
                        <RepoCard repo={repo.clone()} />
                    </div>
                }
            }).collect::<Html>()}
        </section>
    }
}
