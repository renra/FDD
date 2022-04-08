// TODO: Move to its own file
pub mod use_state_with_enum {
  use yew::{html, function_component, use_effect, use_state};

  pub enum RequestState {
    NotAsked,
    Loading,
    Success,
    Failure
  }

  #[function_component(App)]
  pub fn app() -> Html {
    let data_state = use_state(|| RequestState::NotAsked);

    {
      use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
          // Uncomment to make the page load forever
          //data_state.set(RequestState::Loading);

          ()
        });
        || ()
      });
    }

    html! {
      <>
        <h1>{ "FDD - Yew" }</h1>
        <div>{ "Strange behaviour of use_state" }</div>
      </>
    }
  }
}


fn main() {
    yew::start_app::<use_state_with_enum::App>();
}
