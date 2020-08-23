use crate::api::common::FetchCallback;
use crate::models::Developer;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request};

pub fn get_developers(callback: FetchCallback<Vec<Developer>>) -> FetchTask {
    let req = Request::get("/database/developer.json")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
