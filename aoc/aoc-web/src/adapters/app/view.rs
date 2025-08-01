use std::collections::HashMap;

use sycamore::{futures::spawn_local, prelude::*};
use web_sys::wasm_bindgen::JsValue;

use crate::{
    adapters::app::challenge::ChallengeView,
    components::challenge::{ChallengeId, ChallengeInput},
    error::WebResult,
    ports::SolvingService,
};

#[component]
pub fn AppView<S>(solver: S) -> View
where
    S: SolvingService,
{
    let challenges: Signal<HashMap<ChallengeId, ChallengeInput>> = create_signal(HashMap::new());
    let solutions: Signal<HashMap<ChallengeId, ReadSignal<Option<WebResult<_>>>>> =
        create_signal(HashMap::new());
    let derived_solutions = move || {
        challenges
            .get_clone()
            .into_iter()
            .map(|(id, input)| {
                if let Some(solution) = solutions.with(|solutions| solutions.get(&id).cloned()) {
                    return (id, solution.to_owned());
                }

                let (getter, setter) = create_signal(None).split();
                solutions.update(|solutions| solutions.insert(id, getter));

                spawn_local({
                    let solver = solver.to_owned();

                    async move {
                        let solution = solver.solve(input).await;
                        setter(Some(solution));
                    }
                });

                (id, getter)
            })
            .collect::<Vec<_>>()
    };

    let dialog_ref = create_node_ref();
    let open_challenge_form = move |_| {
        let dialog_js: JsValue = dialog_ref.get().into();
        let dialog_elem: web_sys::HtmlDialogElement = dialog_js.into();
        let _ = dialog_elem.show_modal();
    };

    let challenge_input = create_signal(String::default());
    let submit_challenge = move |_| {
        challenges.update(|challenges| {
            let id = ChallengeId::default();
            challenges.insert(id, ChallengeInput::new(challenge_input.take()));
        });
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
                    list=derived_solutions,
                    view={
                        |(_, solution)| {
                            view! {
                                li(class="challenge-item") {
                                    ChallengeView(solution=solution)
                                }
                            }
                        }
                    },
                    key=|(id, _)| *id
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
