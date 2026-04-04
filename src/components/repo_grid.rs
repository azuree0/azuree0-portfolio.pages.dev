use crate::components::RepoCard;
use crate::models::Repo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RepoGridProps {
    pub repos: Vec<Repo>,
    pub show_icosahedron: bool,
}

/// Renders a grid of RepoCard components. First card shows icosahedron overlay when show_icosahedron is true.
#[function_component(RepoGrid)]
pub fn repo_grid(props: &RepoGridProps) -> Html {
    html! {
        <section class="repo-grid">
            {props.repos.iter().enumerate().map(|(i, repo)| {
                let show_icosahedron = i == 0 && props.show_icosahedron;
                let image_lazy = i > 0;
                html! {
                    <div key={repo.name.clone()} class="scroll-reveal">
                        <RepoCard repo={repo.clone()} show_icosahedron={show_icosahedron} image_lazy={image_lazy} />
                    </div>
                }
            }).collect::<Html>()}
        </section>
    }
}
