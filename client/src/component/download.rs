use crate::Msg;
use seed::{prelude::*, *};

fn download() -> Node<Msg> {
    div![button![
        C!["button"],
        attrs! {
            At::Href => format![
                ""
            ]
        }
    ]]
}
