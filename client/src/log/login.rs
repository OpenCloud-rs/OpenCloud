use seed::{prelude::*, *};

use crate::{Model, Msg};

pub fn login(_model: &Model) -> Node<Msg> {
    div![
        form![
            input![
                attrs! {At::Type => "text", At::Name => "name"},
                input_ev(Ev::Input, |e| Msg::InputChange(e, crate::InputType::Name))
            ],
            input![
                attrs! {At::Type => "text", At::Name => "password"},
                input_ev(Ev::Input, |e| Msg::InputChange(
                    e,
                    crate::InputType::Password
                ))
            ]
        ],
        button!["hello", ev(Ev::Click, |_| Msg::Connect)]
    ]
}
