use crate::Msg;
use seed::{prelude::*, *};

pub fn footer() -> Node<Msg> {
    footer![
        C!["footer"],
        div![
            C!["content has-text-centered"],
            p![
                strong!["OpenCloud"],
                format![" by "],
                a![format!["Rheydskey"]],
                format![". The source code is licensed "],
                a![
                    attrs! {At::Href => format!["https://opensource.org/licenses/GPL-3.0"]},
                    format!["GPL v3"]
                ],
            ]
        ]
    ]
}
