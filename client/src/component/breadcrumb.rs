use crate::Msg;
use seed::{prelude::*, *};

pub fn breadcrumb(url: String) -> Node<Msg> {
    let vec: Vec<&str> = url.split('/').collect();
    let mut n = 0;
    nav![
        C!["breadcrumb is-centered bd-snippet notification has-text-black has-background-link-light"],
            ul![
                li![
                        format!{""}
                    ],
                vec.iter().map( |t| if t.is_empty() {
                    if n == 0 {
                        li![""]    
                    } else {
                        empty![]
                    }
                } else {
                    n+= 1;
                    li![
                        a![
                            t.to_string()
                          ]
                        ]
                })
                ]
        ]
}
