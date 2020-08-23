use crate::api::common::FetchCallback;
use crate::models::developer::Developer;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request};

pub const DEVELOPER_JSON: &str =
    "https://raw.githubusercontent.com/rust-lang-ve/developers/main/database/developer.json";

pub fn get_developers(callback: FetchCallback<Vec<Developer>>) -> FetchTask {
    let req = Request::get(DEVELOPER_JSON).body(Nothing).unwrap();

    FetchService::fetch(req, callback).unwrap()
}
