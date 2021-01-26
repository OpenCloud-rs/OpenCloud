use crate::component::footer::footer;
use account::{login::login, signup::signup};
use component::uploadfile::get_name_of_file;
use http::{get::refresh::refresh, post::create_user::create_user};
use shared::{FType, JsonStruct};
mod account;
mod component;
mod http;
mod library;

use crate::component::breadcrumb::breadcrumb;
use crate::component::uploadfile::upload_file;
use crate::http::get::connect::get_connect;
use crate::http::get::get_files::{back, get_files};
use crate::library::lib::download;
use library::lib::Account;
use seed::{browser::Url, prelude::web_sys::File};
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum StateApp {
    Login,
    SignUp,
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
        name: String::new(),
        pass: String::new(),
        mail: String::new(),
        account: Account::new(),
        token: String::new(),
        state: StateApp::Login,
        route: "".to_string(),
        delete: (false, "".to_string()),
        file: File::new_with_str_sequence(&JsValue::from_str(&"Hello"), "Default"),
        notification: Vec::new(),
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
    pub notification: Vec<(bool, String)>,
    pub upload_toggle: component::uploadfile::State,
    pub name: String,
    pub pass: String,
    pub mail: String,
    pub account: Account,
    pub token: String,
    pub state: StateApp,
    pub route: String,
    pub delete: (bool, String),
    pub file: Result<File, seed::prelude::JsValue>,
}

// ------ ------
//  After Mount
// ------ ------
pub enum InputType {
    Name,
    Password,
    Mail,
}
// ------ ------
//    Update
// ------ ------
pub enum Msg {
    Fetched(Option<JsonStruct>),
    InputChange(String, InputType),
    AddNotification(bool, String),
    RemoveNotification(i32),
    Connect,
    Refresh,
    ChangeState(StateApp),
    Token(String),
    ChangeRoute(String, ChangeRouteType),
    DeleteFile(Result<u16, u16>, String),
    CallDelete(String),
    CallDownload(String),
    SignUp,
    CallSignUp,
    FileSelect(File),
    CallUploadFile,
    CallbackUploadFile(bool, String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetched(Some(folder)) => model.api = folder,
        Msg::Fetched(_) => {
            error!(format!("Fetch error - Fetching folder info failed",));
            orders.skip();
        }
        Msg::ChangeState(e) => model.state = e,
        Msg::InputChange(e, it) => match it {
            InputType::Name => model.account.name = e,
            InputType::Password => model.account.password = e,
            InputType::Mail => model.account.mail = Some(e),
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
        Msg::CallDownload(e) => {
            orders
                .skip()
                .perform_cmd(download(e, "zip".to_string(), model.clone().token));
        }
        Msg::DeleteFile(result, name) => {
            let mut re = (false, name);
            if result.is_ok() {
                re.0 = true;
                orders.skip().perform_cmd(refresh());
            }
            model.notification.push(re);
        }
        Msg::SignUp => {
            model.account = Account::new();
            model.state = StateApp::SignUp;
        }
        Msg::CallSignUp => {
            orders
                .skip()
                .perform_cmd(create_user(model.account.clone()));
        }
        Msg::FileSelect(e) => model.file = Ok(e),
        Msg::CallUploadFile => {
            orders.skip().perform_cmd(http::post::upload::upload_file(
                model.token.clone(),
                model.file.clone().unwrap(),
                model.route.clone()
            ));
        }
        Msg::CallbackUploadFile(e, msg) => {
            log!(format! {"{} / {}",e , msg});
            orders.skip().perform_cmd(refresh());
        }
        Msg::AddNotification(status, content) => {
            model.notification.push((status, content))
        }
        Msg::RemoveNotification(index) => {
            model.notification.remove(index as usize);
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
            vec![div![C!["container is-align-items-center is-flex is-justify-content-center is-align-content-center"], login()]]
        }
        StateApp::SignUp => {
            vec![
                div![C!["container is-align-items-center is-flex is-justify-content-center is-align-content-center"], signup()]]
        }
        StateApp::Logged => {
            let mut notifs: Vec<Node<Msg>> = Vec::new();
            let mut n = 0;
            for i in model.notification.clone() {
                let child = match i.0 {
                    true => {
                        div![
                            C!["notification is-success"],
                            button![
                                C!["delete"],
                                ev(Ev::Click, move |_| Msg::RemoveNotification(n.clone()))
                            ],
                            i.1
                        ]
                    }
                    false => {
                        div![
                            C!["notification is-danger"],
                            button![
                                C!["delete"],
                                ev(Ev::Click, move |_| Msg::RemoveNotification(n.clone()))
                            ],
                            i.1
                        ]
                    }
                };
                notifs.push(child);
                n+=1;
            }

            vec![
                div![
                    attrs! {At::Id => "wrapper"},
                    div![
                        C!["container"],
                        div![
                            C!["column"],
                            notifs,
                            breadcrumb((&model.route).parse().unwrap()),
                            div![
                                C!["columns has-text-centered"],
                                div![C!["column"], upload_file(get_name_of_file(&model.file),&model.route),],
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
