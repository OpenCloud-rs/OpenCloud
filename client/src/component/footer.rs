use crate::Msg;
use seed::{prelude::*, *};

pub fn footer() -> Node<Msg> {
    footer![
        C!["footer"],
        div![
            C!["content has-text-centered"],
            p![
                strong!["OpenCloud"],
                " by ".to_string(),
                a!["Rheydskey"],
                ". The source code is licensed ",
                a![
                    attrs! {At::Href => "https://opensource.org/licenses/GPL-3.0"},
                    "GPL v3"
                ],
            ]
        ]
    ]
}
