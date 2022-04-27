use yew::{ContextProvider, html, function_component, use_context};

#[derive(Clone, Debug, PartialEq)]
struct GlobalState {
    state: String,
}

#[function_component(App)]
pub fn app() -> Html {
  let ctx = GlobalState {
    state: String::from("Loading something very useful")
  };

  html! {
    <>
      <h1>{ "FDD - Yew" }</h1>

      <ContextProvider<GlobalState> context={ctx}>
        <SomeInnerComponent />
      </ContextProvider<GlobalState>>
    </>
  }
}

#[function_component(SomeInnerComponent)]
pub fn some_inner_component() -> Html {
  // In case the context provider is omitted by mistake or intention
  // The following will produce a runtime error
  // Use pattern matching to handle both cases gracefully
  //
  // !!!Don't do!!!
  // let global_state = use_context::<GlobalState>().expect("no ctx found");
  // html! {
  //   <div>{ format!( "So the value from the global state is: {}", global_state.state ) }</div>
  // }

  let global_state = use_context::<GlobalState>();

  match global_state {
    None => html! {
      <div>{ "Did not receive context" }</div>
    },

    Some(global_state_) => {
      html! {
        <div>{ format!( "So the value from the global state is: {}", global_state_.state ) }</div>
      }
    }
  }

}
