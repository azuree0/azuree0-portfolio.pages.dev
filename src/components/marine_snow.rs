// Marine snow: organic particles drifting down through the water column
use yew::prelude::*;

const PARTICLE_COUNT: usize = 50;

#[function_component(MarineSnow)]
pub fn marine_snow() -> Html {
    let particles = (0..PARTICLE_COUNT).map(|i| {
        let left = (i * 37) % 100;
        let size = 1 + (i % 3);
        let duration = 10 + (i % 14) as u32;
        let delay = (i * 23) % 25;
        let drift = ((i * 17) % 41) as i32 - 20;
        html! {
            <span
                class="marine-snow-particle"
                style={format!(
                    "left: {}%; width: {}px; height: {}px; animation-duration: {}s; animation-delay: -{}s; --drift: {}px;",
                    left,
                    size,
                    size,
                    duration,
                    delay,
                    drift
                )}
            />
        }
    });

    html! {
        <div class="marine-snow" aria-hidden="true">
            {particles.collect::<Html>()}
        </div>
    }
}
