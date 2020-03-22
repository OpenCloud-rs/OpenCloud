#![allow(clippy::large_enum_variant)]

use seed::{browser::service::fetch, prelude::*, *};
use serde::{Deserialize, Serialize};

const REPOSITORY_URL: &str = "http://127.0.0.1:8080/cli/";

#[derive(Serialize)]
struct SendMessageRequestBody {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
struct SendMessageResponseBody {
    pub success: bool,
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Serialize, Deserialize)]
struct API {
    pub result: String,
    pub legnth: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    api: API,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            api: API {
                result: "Loading...".into(),
                legnth: "Loading...".into()
            },
        }
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_repository_info());
    AfterMount::default()
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    RepositoryInfoFetched(fetch::ResponseDataResult<API>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RepositoryInfoFetched(Ok(API)) => model.api = API,

        Msg::RepositoryInfoFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching repository info failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
    }
}

async fn fetch_repository_info() -> Result<Msg,Msg> {
    Request::new(REPOSITORY_URL)
        .fetch_json_data(Msg::RepositoryInfoFetched)
        .await
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        md!["# Repo info"],
        div![format!(
            "Name: {}, SHA: {}",
            model.api.result, model.api.legnth
        )],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}