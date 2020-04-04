
use crate::component::uploadfile::upload_file;
use seed::{browser::service::fetch, prelude::*, *};
use shared::Folder;
use std::default::Default;
mod component;
const REPOSITORY_URL: &str = "http://127.0.0.1:8080/cli/";
// ------ ------
//     Model
// ------ ------

#[derive(Debug)]
struct Model {
    pub api: Folder,
    pub uri: String,
    pub upload_toggle: component::uploadfile::State,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            api: Folder {
                result: false,
                lenght: 0,
                content: vec![],
            },
            uri: String::from(""),
            upload_toggle: component::uploadfile::State::Hidden
        }
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_repository_info(url));
    AfterMount::default()
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    RoutePage(Url),
    Fetched(fetch::ResponseDataResult<Folder>),
    Next
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetched(Ok(folder)) => model.api = folder,
        Msg::Fetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching folder info failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
        Msg::RoutePage(url) => {
            orders.skip().perform_cmd(fetch_repository_info(url.clone()));
            let url_t = url;
            {model.uri = String::from(&url_t.path.into_iter().collect::<Vec<String>>().join("/"));}
        }
        Msg::Next => {
            model.upload_toggle = model.upload_toggle.next()
        }
    }
}

async fn fetch_repository_info(url: Url) -> Result<Msg, Msg> {
    let mut url_string: String = REPOSITORY_URL.to_owned();
    url_string.push_str(&url.path.into_iter().collect::<Vec<String>>().join("/"));
    log!(url_string);

    Request::new(url_string).fetch_json_data(Msg::Fetched).await
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {

div![
        upload_file(model.upload_toggle, &model.uri),

        h2![model.uri],
        md!["# Folder Info"],
        div![format!(
            "Result: {}, Lenght: {},",
            model.api.result, model.api.lenght
            )],
        h4!["Content info"],
        component::folder_list::folder_list(model.api.content.clone()),
        ]

}

fn routes(url: Url) -> Option<Msg> {
    Some(Msg::RoutePage(url))
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
