use sycamore::prelude::*;
use web_sys::wasm_bindgen::JsValue;

use crate::{
    adapters::app::challenge::ChallengeView, archetypes::Challenge,
    components::challenge::ChallengeInput, ports::SolvingService,
};

#[component]
pub fn AppView<S>(solver: S) -> View
where
    S: SolvingService,
{
    let challenges = create_signal(Vec::new());
    let dialog_ref = create_node_ref();
    let open_challenge_form = move |_| {
        let dialog_js: JsValue = dialog_ref.get().into();
        let dialog_elem: web_sys::HtmlDialogElement = dialog_js.into();
        let _ = dialog_elem.show_modal();
    };

    let challenge_input = create_signal(String::default());
    let submit_challenge = move |_| {
        let input = ChallengeInput::new(challenge_input.take());
        let challenge = Challenge::new(input, solver.clone());
        challenges.update(|challenges| challenges.push(challenge));
    };

    view! {
        div(class="app") {
            div(class="menu") {
                button(class="btn-new-challenge", "command"="show-modal", "commandfor"="challenge-form-modal", "type"="button", on:click=open_challenge_form) {
                    p {
                        "Solve challenge"
                    }
                    i(class="iconoir-key-plus") {}
                }
            }
            ul(class="challenges") {
                Keyed(
                    list=challenges,
                    view={
                        |challenge| {
                            let solution = challenge.get_solution();

                            view! {
                                li(class="challenge-item") {
                                    ChallengeView(solution=solution)
                                }
                            }
                        }
                    },
                    key=|challenge| challenge.get_id()
                )
            }
        }
        dialog(r#ref=dialog_ref, class="challenge-form", id="challenge-form-modal", "closedby"="any") {
            form(autocomplete="off", method="dialog", on:submit=submit_challenge) {
                div(class="challenge-input") {
                    label(r#for="input") { "Challenge input" }
                    textarea(id="input", name="input", rows="5", cols="10", "autofocus"="true", "autocorrect"="off", required=true, wrap="off", bind:value=challenge_input) {}
                }
                button(class="btn-submit-challenge") {
                    p {
                        "Solve"
                    }
                    i(class="iconoir-send-solid") {}
                }
            }
        }
    }
}
