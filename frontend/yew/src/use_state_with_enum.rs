use yew::{html, function_component, use_effect, use_state};
use reqwasm::http::{Request};

pub mod use_state_with_enum {
  pub enum RequestState {
    NotAsked,
    Loading,
    Fetched,
    Error
  }

  #[function_component(App)]
  pub fn app() -> Html {
    let data_state = use_state(|| RequestState::NotAsked);

    {
      use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
          data_state.set(RequestState::Loading);

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

