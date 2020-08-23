use crate::api;
use crate::components::ListItem;
use crate::models::developer::Developer;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    developers: Vec<Developer>,
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
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let developers: Vec<Developer> = Vec::new();

        link.send_message(Msg::FetchDevelopers);

        Self {
            state: State {
                developers,
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

                let handler = self.link.callback(
                    move |response: api::common::FetchResponse<Vec<Developer>>| {
                        let (_, Json(data)) = response.into_parts();

                        match data {
                            Ok(developers) => Msg::FetchDevelopersSuccess(developers),
                            Err(err) => Msg::FetchDevelopersError(err),
                        }
                    },
                );

                self.task = Some(api::developers::get_developers(handler));

                true
            }
            Msg::FetchDevelopersSuccess(developers) => {
                self.state.developers = developers;
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
            .developers
            .iter()
            .map(|developer| {
                // handle option values
                let avatar_url = match &developer.avatar_url {
                    Some(avatar_url) => avatar_url.clone(),
                    None => String::default(),
                };

                let github = match &developer.github {
                    Some(username) => username.clone(),
                    None => String::default(),
                };

                html! {
                    <ListItem
                        avatar_url=avatar_url
                        full_name=developer.full_name().as_str()
                        email=developer.email.as_str()
                        github=github
                    />
                }
            })
            .collect();

        if !self.state.fetch_developers_loaded {
            html! {
                <span>{"Loading..."}</span>
            }
        } else if self.state.fetch_developers_error.is_some() {
            html! {
                <div>
                    <span>{"Something came wrong!"}</span>
                </div>
            }
        } else {
            html! {
                <ol id="developer-list">{developers}</ol>
            }
        }
    }
}
