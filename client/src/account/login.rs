use seed::{prelude::*, *};

use crate::Msg;

pub fn login() -> Node<Msg> {
    div![
        form![
            input![
                C!["input"],
                attrs! {At::Type => "text", At::Name => "name"},
                input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
            ],
            input![
                C!["input"],
                attrs! {At::Type => "text", At::Name => "password"},
                input_ev(Ev::Input, |e| Msg::InputChange(
                    e,
                    crate::InputType::Password
                ))
            ],
        ],
        button![
            C!["button is-link"],
            "Welcome back",
            ev(Ev::Click, |_| Msg::Connect)
        ],
        button![
            C!["button is-link"],
            "Sign Up",
            ev(Ev::Click, |_| Msg::SignUp)
        ]
    ]
}
