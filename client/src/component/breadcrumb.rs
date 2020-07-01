use crate::Msg;
use seed::{prelude::*, *};

pub fn breadcrumb(url: String) -> Node<Msg> {
    let vec: Vec<&str> = url.split("/").collect();
    nav![
        C!["breadcrumb is-centered bd-snippet notification has-text-black has-background-link-light"],
            ul![
                vec.iter().map( |t|
                    li![
                        a![
                            attrs![At::Href => format!["{}", t]],
                            format!["{}", t]
                          ]
                        ]
                    )
                ]
        ]
}