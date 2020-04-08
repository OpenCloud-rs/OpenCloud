use crate::REPOSITORY_URL;
use crate::Msg;
use seed::{prelude::*, *};
pub fn folder_list(content: Vec<String>) -> Node<Msg> {
    div![table![
        tr![
            td![
                a!["..", attrs! {At::Href => ".."}]
                ]
            ],
            content.iter()
                .map(|t|

                    tr![
                        td![
                            a![format!["{}/", &t.to_string()], attrs! {At::Href => format!["{}/",t.to_string()]}]
                            ],
                        td![
                            button![format!["Delete : {}", t.to_string()], ev(Ev::Click, |t| Msg::Delete(String::from(REPOSITORY_URL.to_owned() + &String::from(&t.to_string()))))]
                        ]
                        ])
    ]]
}
