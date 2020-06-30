use crate::component::footer::footer;
use shared::{FType, JsonStruct};
mod component;
mod library;
use library::lib::delete;
use library::lib::fetch_repository_info;
use seed::{prelude::*, *};
use seed::browser::Url;
use crate::component::breadcrumb::breadcrumb;

const REPOSITORY_URL: &str = "http://127.0.0.1:2000/api/";

// ------ ------
//     Model
// ------ ------

#[derive(Debug)]
struct Model {
    pub api: JsonStruct,
    pub uri: String,
    pub upload_toggle: component::uploadfile::State,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            api: JsonStruct {
                result: false,
                lenght: 0,
                ftype: FType::File,
                content: vec![],
            },
            uri: String::new(),
            upload_toggle: component::uploadfile::State::Hidden,
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
    Fetched(Option<JsonStruct>),
    Next,
    Delete(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetched(Some(folder)) => model.api = folder,
        Msg::Fetched(_) => {
            error!(format!(
                "Fetch error - Fetching folder info failed",
            ));
            orders.skip();
        }
        Msg::RoutePage(url) => {
            orders
                .skip()
                .perform_cmd(fetch_repository_info(url.clone()));
                 model.uri = url.path().to_vec().join("/").clone()
        }
        Msg::Next => model.upload_toggle = model.upload_toggle.next(),
        Msg::Delete(url) => {
            orders.skip().perform_cmd(delete(url));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![
        attrs!{At::Id => format!["wrapper"]},
            div![
                C!["container"],
                div![
                    C!["column"],
                    breadcrumb((&model.uri).parse().unwrap()),
                    component::folder_list::folder_list(model.api.content.clone()),
                    div![
                       format!(
                            "Result: {}, Lenght: {},",
                            model.api.result, model.api.lenght
                        )],
                ]
              ]
        ],
        footer()
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
