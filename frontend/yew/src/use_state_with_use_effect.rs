use yew::{html, function_component, use_effect, use_state, use_state_eq, use_effect_with_deps};

#[derive(PartialEq, Copy, Clone)]
pub enum RequestState {
  NotAsked,
  Loading,
  //Success,
  //Failure
}

pub fn request_state_to_string(state: RequestState) -> String {
  match state {
    // It is tempting to match by NotAsked ... instead of RequestState::NotAsked
    // but that leads to incorrect results
    //
    // !!!Don't do!!!
    // NotAsked => String::from("not asked"),
    // Loading => String::from("loading"),

    RequestState::NotAsked => String::from("not asked"),
    RequestState::Loading => String::from("loading"),
  }
}

#[function_component(App)]
pub fn app() -> Html {
  // use_effect is run on every render and use_state triggers a re-render
  // so setting the state directly here triggers and endless loop
  // If you want the same behaviour as in React you must call use_effect_with_deps
  //
  // !!!Don't do!!!
  //let data_state = use_state(|| RequestState::NotAsked);

  //{
  //  let data_state = data_state.clone();
  //  use_effect(move || {
  //    log::info!("use_effect called");
  //    data_state.set(RequestState::Loading);
  //    || ()
  //  });
  //}

  // Solution #1 use_state_eq => 2 renders, 2 side-effect calls
  //   this may be wrong depending on the use-case
  //let data_state = use_state_eq(|| RequestState::NotAsked);

  //{
  //  let data_state = data_state.clone();
  //  use_effect(move || {
  //    log::info!("use_effect called (with use_state_eq)");
  //    data_state.set(RequestState::Loading);
  //    || ()
  //  });
  //}

  // Solution #2 use_effect_with_deps => 2 renders, 1 side-effect call
  let data_state = use_state(|| RequestState::NotAsked);

  {
    let data_state = data_state.clone();
    use_effect_with_deps(
      move |_| {
        log::info!("use_effect_with_deps called");
        data_state.set(RequestState::Loading);
        || ()
      },
      ()
    );
  }

  log::info!("Rendering");

  html! {
    <>
      <h1>{ "FDD - Yew" }</h1>
      <div>{ format!("The request state is: {}", request_state_to_string(*data_state)) }</div>
    </>
  }
}
