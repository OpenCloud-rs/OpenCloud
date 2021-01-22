use crate::Msg;
use seed::{prelude::*, *};
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
pub fn upload_file(state: State, url: &String) -> Node<Msg> {
    if cfg!(debug_assertions) {
        println!("{}", url);
    }
    // let e = ev(Ev::Input, |e| {let value = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().;Msg::Log(format!{"{:?}", value})});
    div![
        div![
            C!["mt-2 is-centered is-mobile columns"],
            button![
                C!["columns button is-centered"],
                format![
                    "{}",
                    match state {
                        self::State::Hidden => {
                            "Show the upload menu"
                        }
                        self::State::Show => {
                            "Hidden the upload menu"
                        }
                    }
                ],
                ev(Ev::Click, |_| Msg::UploadNext)
            ],
        ],
        div![
            /*form![
                    input![
                        C!["file-input"],
                        attrs!{
                            At::Name => "file"
                            At::Value => "File"
                            At::Type => "file"
                        }
                    ],
                    span!("Hey"),
                    input![
                        C!["button"],
                        attrs!{
                            At::Type => "submit"
                        }
                    ]

            ],*/
            attrs! {At::from("style") => format!["visibility: {}", state]},
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
                button![C!["button is-link"], ev(Ev::Click, |_| Msg::CallUploadFile)]
            ]
        ]
    ]
}
