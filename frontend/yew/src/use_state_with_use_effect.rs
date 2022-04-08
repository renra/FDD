use yew::{html, function_component, use_effect, use_state};

pub enum RequestState {
  NotAsked,
  //Loading,
  //Success,
  //Failure
}

#[function_component(App)]
pub fn app() -> Html {
  let data_state = use_state(|| RequestState::NotAsked);

  {
    use_effect(move || {
      // Not sure why but uncommenting the following line completely freezes the app
      // Looks like and endless loop to me
      //
      // !!!Don't do!!!
      //data_state.set(RequestState::Loading);
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
