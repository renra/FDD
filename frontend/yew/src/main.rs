mod use_state_with_use_effect;
mod use_context;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<use_state_with_use_effect::App>();
}
