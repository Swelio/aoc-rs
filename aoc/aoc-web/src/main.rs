use aoc_web::{
    adapters::{app::App, solver::WebSolver},
    ports::WasmRender,
};

fn main() {
    App::new(WebSolver).render();
}
