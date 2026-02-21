use crate::components::RepoCard;
use crate::models::Repo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepoGridProps {
    pub repos: Vec<Repo>,
    pub show_poem: bool,
}

#[function_component(RepoGrid)]
pub fn repo_grid(props: &RepoGridProps) -> Html {
    html! {
        <section class="repo-grid">
            {props.repos.iter().enumerate().map(|(i, repo)| {
                let show_poem = i == 0 && props.show_poem;
                html! {
                    <div key={repo.name.clone()} class="scroll-reveal">
                        <RepoCard repo={repo.clone()} show_poem={show_poem} />
                    </div>
                }
            }).collect::<Html>()}
        </section>
    }
}
