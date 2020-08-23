use crate::api;
use crate::models::Developer;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    results: Vec<Developer>,
    fetch_developers_error: Option<Error>,
    fetch_developers_loaded: bool,
}

pub enum Msg {
    FetchDevelopers,
    FetchDevelopersSuccess(Vec<Developer>),
    FetchDevelopersError(Error),
}

pub struct Search {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for Search {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let results: Vec<Developer> = Vec::new();

        link.send_message(Msg::FetchDevelopers);

        Self {
            state: State {
                results,
                fetch_developers_error: None,
                fetch_developers_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchDevelopers => {
                self.state.fetch_developers_loaded = false;

                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Developer>>| {
                            let (_, Json(data)) = response.into_parts();

                            match data {
                                Ok(developers) => Msg::FetchDevelopersSuccess(developers),
                                Err(err) => Msg::FetchDevelopersError(err),
                            };
                        });

                self.task = Some(api::get_developers(handler));

                true
            }
            Msg::FetchDevelopersSuccess(developers) => {
                self.state.results = developers;
                self.state.fetch_developers_loaded = true;

                true
            }
            Msg::FetchDevelopersError(error) => {
                self.state.fetch_developers_error = Some(error);
                self.state.fetch_developers_loaded = true;

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let developers: Vec<Html> = self
            .state
            .results
            .iter()
            .map(|developer| {
                html! {
                    <div>
                        <h3>{&developer.first_name} {&developer.last_name}</h3>
                    </div>
                }
            })
            .collect();

        if !self.state.fetch_developers_loaded {
            html! {
                <span>{"Loading..."}</span>
            }
        } else if let Some(_) = self.state.fetch_developers_error {
            html! {
                <div>
                    <span>{"Something came wrong!"}</span>
                </div>
            }
        } else {
            html! {
                <div>{developers}</div>
            }
        }
    }
}
