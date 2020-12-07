use crate::component::footer::footer;
use http::get::refresh::refresh;
use log::login::login;
use shared::{FType, JsonStruct};
mod component;
mod http;
mod library;
mod log;

use crate::component::breadcrumb::breadcrumb;
use crate::component::uploadfile::upload_file;
use crate::http::get::connect::get_connect;
use crate::http::get::get_files::{back, get_files};
use crate::library::lib::download;
use library::lib::delete;
use library::lib::fetch_repository_info;
use library::lib::Account;
use seed::browser::Url;
use seed::prelude::wasm_bindgen::__rt::std::str::FromStr;
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum StateApp {
    Login,
    Logged,
}

pub enum ChangeRouteType {
    Remove,
    Add,
}
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        api: JsonStruct {
            result: false,
            lenght: 0,
            ftype: FType::File,
            content: vec![],
        },
        uri: "".to_string(),
        url: Url::new(),
        upload_toggle: component::uploadfile::State::Hidden,
        dropdown: component::download::State::NotActive,
        modal_toggle: component::delete::State::NotActive,
        name: String::new(),
        pass: String::new(),
        account: Account::new(),
        token: String::new(),
        state: StateApp::Login,
        route: "".to_string(),
        delete: (false, "".to_string()),
    }
}
// ------ ------
//     Model
// ------ ------

#[derive(Debug, Clone)]
pub struct Model {
    pub api: JsonStruct,
    pub uri: String,
    pub url: Url,
    pub upload_toggle: component::uploadfile::State,
    pub dropdown: component::download::State,
    pub modal_toggle: component::delete::State,
    pub name: String,
    pub pass: String,
    pub account: Account,
    pub token: String,
    pub state: StateApp,
    pub route: String,
    pub delete: (bool, String),
}

// ------ ------
//  After Mount
// ------ ------
pub enum InputType {
    Name,
    Password,
}
// ------ ------
//    Update
// ------ ------
pub enum Msg {
    RoutePage(Url),
    Fetched(Option<JsonStruct>),
    UploadNext,
    DropdownNext,
    ModalToggle,
    Download(String),
    Delete(Url),
    InputChange(String, InputType),
    Connect,
    Refresh,
    Token(String),
    ChangeRoute(String, ChangeRouteType),
    DeleteFile(Result<u16, u16>, String),
    CallDelete(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetched(Some(folder)) => model.api = folder,
        Msg::Fetched(_) => {
            error!(format!("Fetch error - Fetching folder info failed",));
            orders.skip();
        }
        Msg::RoutePage(url) => {
            orders
                .skip()
                .perform_cmd(fetch_repository_info(url.clone()));
            model.uri = url.path().to_vec().join("/").clone()
        }
        Msg::UploadNext => model.upload_toggle = model.upload_toggle.next(),
        Msg::DropdownNext => model.dropdown = model.dropdown.next(),
        Msg::ModalToggle => model.modal_toggle = model.modal_toggle.next(),
        Msg::Download(dtype) => {
            orders
                .skip()
                .perform_cmd(download(model.url.clone(), dtype));
        }
        Msg::Delete(url) => {
            orders.skip().perform_cmd(delete(url));
        }
        Msg::InputChange(e, it) => match it {
            InputType::Name => model.account.name = e,
            InputType::Password => model.account.password = e,
        },
        Msg::Connect => {
            orders
                .skip()
                .perform_cmd(get_connect(model.clone().account));
        }
        Msg::Token(e) => {
            if e == "No user was found" {
                model.token = "No user was found".to_string();
            } else {
                model.token = e.clone();
                model.state = StateApp::Logged;
                orders
                    .skip()
                    .perform_cmd(get_files("".to_string(), e.clone()));
            }
        }
        Msg::Refresh => {
            orders
                .skip()
                .perform_cmd(get_files(model.clone().route, model.clone().token));
        }
        Msg::ChangeRoute(s, crt) => {
            match crt {
                ChangeRouteType::Remove => {
                    model.route = back(model.clone().route);
                }
                ChangeRouteType::Add => {
                    model.route.push_str(format!("{}/", s).as_str());
                }
            };
            orders
                .skip()
                .perform_cmd(get_files(model.clone().route, model.clone().token));
        }
        Msg::CallDelete(e) => {
            orders
                .skip()
                .perform_cmd(http::delete::delete::delete(model.clone().token, e));
        }
        Msg::DeleteFile(result, name) => {
            let mut re = (false, name);
            if result.is_ok() {
                re.0 = true;
                orders.skip().perform_cmd(refresh());
            }
            model.delete = re;
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    log!(model.route);
    match model.state {
        StateApp::Login => {
            vec![div![C!["container"], login(&model.clone())]]
        }
        StateApp::Logged => {
            let delete = if model.delete.1.is_empty() {
                div![""]
            } else {
                match model.delete.0 {
                    true => {
                        div![
                            C!["notification is-success"],
                            button![
                                C!["delete"],
                                ev(Ev::Click, |_| Msg::DeleteFile(Err(1), "".to_string()))
                            ],
                            format! {"Delete succesfully"}
                        ]
                    }
                    false => {
                        div![
                            C!["notification is-danger"],
                            button![C!["delete"],],
                            format! {"Delete unsuccessfully"}
                        ]
                    }
                }
            };
            vec![
                div![
                    attrs! {At::Id => "wrapper"},
                    div![
                        C!["container"],
                        div![
                            C!["column"],
                            delete,
                            breadcrumb((&model.route).parse().unwrap()),
                            div![
                                C!["columns has-text-centered"],
                                div![C!["column"], upload_file(model.upload_toggle, &model.route),],
                                div![
                                    C!["column"],
                                    component::delete::delete(
                                        model.modal_toggle,
                                        Url::from_str(model.clone().route.as_str()).unwrap()
                                    ),
                                ],
                                div![C!["column"], component::download::download(model.dropdown)]
                            ],
                            component::folder_list::folder_list(
                                model.api.content.clone(),
                                model.route.clone()
                            ),
                        ]
                    ]
                ],
                footer(),
            ]
        }
    }
}
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
