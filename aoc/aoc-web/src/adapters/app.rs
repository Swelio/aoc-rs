mod challenge;
mod view;

use crate::{
    adapters::{app::view::AppView, solver::WebSolver},
    ports::{SolvingService, WasmRender},
};

#[derive(Debug, derive_more::Constructor)]
pub struct App<S = WebSolver>(S);

impl<S: SolvingService> WasmRender for App<S> {
    fn render(&self) {
        sycamore::render(|| AppView(self.0.to_owned()));
    }
}
