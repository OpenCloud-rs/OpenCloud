use crate::Msg;
use crate::REPOSITORY_URL;
use seed::{prelude::*, *};
use shared::Folder;

pub fn folder_list(content: Vec<Folder>) -> Node<Msg> {
    div![table![
        tr![
            td![
                a!["..", attrs! {At::Href => ".."}]]],
                content.iter().map(|t| tr![
            td![
                a![
                    format!["{}/", &t.name.to_string()],
                    attrs! {At::Href => format!["{}/",t.name.to_string()]}
                ]
            ],
            td![
                button![
                    format!["Delete : {}", t.name.to_string()],
                    ev(Ev::Click, |t| Msg::Delete(
                        String::from(
                            REPOSITORY_URL.to_owned() + &String::from(&t.to_string())
                        )
                       )
                    )
            ]
            ]
        ])
    ]]
}
