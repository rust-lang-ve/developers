use yew::prelude::*;

use crate::components::Home;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
              <header>
                <h1>{"Developers"}</h1>
              </header>
              <main>
                <Home />
              </main>
            </div>
        }
    }
}
