use yew::prelude::*;

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header id="app-header">
                <div>
                    <h1>{"Developers"}</h1>
                    <span>{"Directory of developers to help people interested to find and publish profiles"}</span>
                </div>
                <img
                    src="https://camo.githubusercontent.com/5d93f4ade6f0224be9ea1fbe769b2b5a143e2c75/68747470733a2f2f7669676e657474652e77696b69612e6e6f636f6f6b69652e6e65742f6e79616e6361742f696d616765732f642f64332f4e79616e2d6361742e6769662f7265766973696f6e2f6c61746573742f7363616c652d746f2d77696474682d646f776e2f3334303f63623d323031333132333132323235303026706174682d7072656669783d6573"
                    width="50"
                    height="38"
                />
            </header>
        }
    }
}
