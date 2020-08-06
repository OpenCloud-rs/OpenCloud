use crate::library::lib;
use crate::Msg;
use seed::Url;
use seed::{prelude::*, *};
use std::fmt;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Active,
    NotActive,
}
impl State {
    pub fn next(self) -> Self {
        match self {
            Self::Active => Self::NotActive,
            Self::NotActive => Self::Active,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::NotActive
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::Active => "is-active",
            Self::NotActive => "",
        };
        write!(f, "{}", state)
    }
}
pub fn delete(state: State, url: Url) -> Node<Msg> {
    div![
        button![
            C!["button"],
            "Delete this folder",
            ev(Ev::Click, |_| Msg::ModalToggle)
        ],
        div![
            C![format![
                "modal {}",
                match state {
                    State::NotActive => {
                        ""
                    }
                    State::Active => {
                        "is-active"
                    }
                }
            ]],
            div![C!["modal-background"]],
            div![
                C!["modal-content"],
                div![C!["box"], "Do you want to delete this folder"],
                div![
                    C!["button is-danger"],
                    ev(Ev::Click, |_| Msg::Delete(url)),
                    ev(Ev::Click, |_| Msg::ModalToggle),
                    "Delete this folder"
                ]
            ],
            div![
                C!["modal-close is-large"],
                ev(Ev::Click, |_| Msg::ModalToggle)
            ],
        ]
    ]
}
