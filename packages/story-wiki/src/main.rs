use story_wiki::components::main::Main;

#[cfg(debug_assertions)]
fn init_logging() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
}

#[cfg(not(debug_assertions))]
fn init_logging() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
}

fn main() {
    init_logging();
    yew::Renderer::<Main>::new().render();
}
