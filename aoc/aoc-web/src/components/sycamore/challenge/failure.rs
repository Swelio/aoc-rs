use sycamore::prelude::*;

use crate::error::WebError;

#[component]
pub fn ChallengeFailure(err: WebError) -> View {
    view! {
        p { "Error occurred while processing challenge input: " (err.to_string()) }
    }
}
