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
    println!("{}", url);
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
            ev(Ev::Click, |_| Msg::Next)
        ],
    ],
    div![

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
                ],
                span![
                    C!["file-cta"],
                    span![
                        C!["file-label"],
                        "Choose a file"
                    ]
                ]
          ]
        ]
            /*form![
                attrs! {
                    At::from("method") => "post",
                    At::from("enctype") => "multipart/form-data",
                    At::from("action") => format!["{}", url],
                },
                input![
                C!["button"],
                attrs! {
                    At::from("name") => "file",
                    At::from("type") => "file",
                }],
                input![
                C!["button"],
                attrs! {
                    At::from("type") => "submit",
                }]
            ]*/
        ]
    ]
}
