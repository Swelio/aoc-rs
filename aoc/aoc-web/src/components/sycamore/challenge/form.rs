use sycamore::{prelude::*, web::events::SubmitEvent};

use crate::adapters::sycamore::ChallengeId;

#[component(inline_props)]
pub fn ChallengeForm<F>(id: ChallengeId, submit: F) -> View
where
    F: Fn(String) + Copy + 'static,
{
    let input = create_signal(String::new());

    let input_id = format!("challenge-input-{id}");
    let label_id = input_id.clone();

    view! {
        form(on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            submit(input.take());
        }) {
            div {
                label("for"=label_id) {
                    "Challenge input"
                }
                textarea(id=input_id, bind:value=input) {}
            }
            button("type"="submit") {
                "Solve"
            }
        }
    }
}
