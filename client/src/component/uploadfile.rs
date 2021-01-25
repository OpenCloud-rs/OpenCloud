use crate::Msg;
use seed::{*, prelude::{*, web_sys::File}};
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Show,
    Hidden,
}
impl State {
    pub fn next(self) -> Self {
        match self {
            Self::Show => Self::Hidden,
            Self::Hidden => Self::Show,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Hidden
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::Show => "visible",
            Self::Hidden => "hidden",
        };
        write!(f, "{}", state)
    }
}
pub fn get_name_of_file(file: &Result<File, seed::prelude::JsValue>) -> String {
    if let Ok(e) = file {
        e.name()
    } else {
        String::new()
    }
}
pub fn upload_file(file_name: String, url: &String) -> Node<Msg> {
    if cfg!(debug_assertions) {
        println!("{}", url);
    }
    let button = if !file_name.is_empty() {
        button![C!["button is-link"], ev(Ev::Click, |_| Msg::CallUploadFile),  format!("Upload : {}", file_name)]
    } else {
        span![]
    };

    div![
        div![
            div![
                C!["file columns is-centered"],
                label![
                    C!["file-label"],
                    input![
                        C!["file-input"],
                        attrs! {
                            At::from("name") => "file",
                            At::from("type") => "file",
                        },
                        ev(Ev::Input, |e| {
                            let value = e
                                .target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .files()
                                .unwrap()
                                .get(0)
                                .unwrap();
                            Msg::FileSelect(value)
                        })
                    ],
                    span![C!["file-cta"], span![C!["file-label"], "Choose a file"]]
                ],
                button
            ]
        ]
    ]
}
