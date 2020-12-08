use seed::{prelude::*, *};

use crate::Msg;

pub fn signup() -> Node<Msg> {
    div![
        C!["container"],
        form![
            input![
                C!["input"],
                attrs! {At::Type => "text", At::Name => "name", At::Placeholder => "Name"},
                input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
            ],
            input![
                C!["input"],
                attrs! {At::Type => "email", At::Name => "email",At::Placeholder => "email"},
                //input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
            ],
            input![
                C!["input"],
                attrs! {At::Type => "password", At::Name => "password",At::Placeholder => "password"},
                // input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
            ],
            button![
                C!["button is-link"],
                "Sign Up",
                ev(Ev::Click, |_| Msg::SignUp)
            ]
        ]
    ]
}