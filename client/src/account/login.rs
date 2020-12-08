use seed::{prelude::*, *};

use crate::Msg;

pub fn login() -> Node<Msg> {
    div![
            h1![C!["title"],"Welcome on your OpenCloud Server"],
            form![
                input![
                    C!["input mt-2"],
                    attrs! {At::Type => "text", At::Name => "name"},
                    input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
                ],
                input![
                    C!["input mt-2"],
                    attrs! {At::Type => "text", At::Name => "password"},
                    input_ev(Ev::Input, |e| Msg::InputChange(
                        e,
                        crate::InputType::Password
                    ))
                ],
            ],
            div![
                C!["level mt-2"],
                div![
                    C!["level-left"],
                    button![
                        C!["button is-link"],
                        "Welcome back",
                        ev(Ev::Click, |_| Msg::Connect)
                    ]
                ],
                div![
                    C!["level-right"],
                    button![
                        C!["button is-white"],
                        "Sign Up",
                        ev(Ev::Click, |_| Msg::SignUp)
                    ]
                ]
            ],
    ]
}
