#![allow(clippy::large_enum_variant)]

use seed::{browser::service::fetch, prelude::*, *};
use serde::{Deserialize, Serialize};
use shared::Folder;
const REPOSITORY_URL: &str = "http://127.0.0.1:8080/cli/";

// ------ ------
//     Model
// ------ ------


#[derive(Debug, Serialize, Deserialize)]
struct Model {
    api: Folder,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            api: Folder {
                result: false ,
                lenght: 0,
                content: vec![]
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
    RepositoryInfoFetched(fetch::ResponseDataResult<Folder>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RepositoryInfoFetched(Ok(Folder)) => model.api = Folder,

        Msg::RepositoryInfoFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching folder info failed - {:#?}",
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
        md!["# Folder Info"],
        div![format!(
            "Result: {}, Lenght: {}",
            model.api.result, model.api.lenght
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