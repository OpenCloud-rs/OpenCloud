#![allow(clippy::large_enum_variant)]

use seed::{browser::service::fetch, prelude::*, *};
use serde::{Deserialize, Serialize};
use shared::Folder;

mod component;
const REPOSITORY_URL: &str = "http://127.0.0.1:8080/cli/";
// ------ ------
//     Model
// ------ ------


#[derive(Debug, Serialize, Deserialize)]
struct Model {
    api: Folder,
    uri: String
}

impl Default for Model {
    fn default() -> Self {
        Self {
            api: Folder {
                result: false ,
                lenght: 0,
                content: vec![]
            },
            uri: String::from("")
        }
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_repository_info(REPOSITORY_URL));
    AfterMount::default()
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    RoutePage(String),
    RepositoryInfoFetched(fetch::ResponseDataResult<Folder>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RepositoryInfoFetched(Ok(folder)) => model.api = folder,

        Msg::RepositoryInfoFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching folder info failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
        Msg::RoutePage(url) => {
            model.uri = url;
        }
    }
}

async fn fetch_repository_info(add_url: &str) -> Result<Msg,Msg> {
    let mut url_string : String = REPOSITORY_URL.to_owned();
    url_string.push_str(add_url);
    Request::new(url_string)
        .fetch_json_data(Msg::RepositoryInfoFetched)
        .await
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {

    nodes![
        h2!["{}", model.uri]
        md!["# Folder Info"],
        div![format!(
            "Result: {}, Lenght: {},",
            model.api.result, model.api.lenght
        )],
        h4!["Content info"],
        component::component::folder_list(model.api.content.clone())

    ]

}

fn routes(url: Url) -> Option<Msg>{
    if url.path.is_empty() {
        println!();
    }
    let url_string =  url.path.into_iter().collect::<Vec<String>>().join("/");

    Some(Msg::RoutePage(url_string))

}

//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .routes(routes)
        .after_mount(after_mount)
        .build_and_start();
}