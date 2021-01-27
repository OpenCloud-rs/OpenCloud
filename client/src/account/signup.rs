use seed::{prelude::*, *};

use crate::{Msg, StateApp};

pub fn signup() -> Node<Msg> {
    div![
        C!["container"],
        button![
            C!["button mt-2 is-link"],
            "Back",
            ev(Ev::Click, |_| Msg::ChangeState(StateApp::Login))
        ],
        form![
            input![
                C!["input mt-2"],
                attrs! {At::Type => "text", At::Id => "signup-name", At::Placeholder => "Name"},
                input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
            ],
            input![
                C!["input mt-2"],
                attrs! {At::Type => "email", At::Id => "signup-email",At::Placeholder => "email"},
                input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Mail))
            ],
            input![
                C!["input mt-2"],
                attrs! {At::Type => "password", At::Id => "signup-password",At::Placeholder => "password"},
                input_ev(Ev::Input, |e| Msg::InputChange(
                    e,
                    crate::InputType::Password
                ))
            ],
        ],
        button![
            C!["button mt-2 is-link"],
            "Sign Up",
            ev(Ev::Click, |_| Msg::CallSignUp)
        ]
    ]
}
