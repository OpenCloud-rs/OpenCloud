use crate::Msg;
use seed::{prelude::*, *};
use shared::Folder;

pub fn folder_list(mut content: Vec<Folder>) -> Node<Msg> {

    content.sort();
    div![table![C!["table is-hoverable is-fullwidth"],
        thead![
            tr![
                th![""],
                th!["Name"],
                th!["Type"],
            ],
        ],
        tbody![
            tr![th![],th![a!["..", attrs! {At::Href => ".."}]], th!["Folder"]],
            content.iter().map(|t|
            tr![
                th![
                if t.ftype.to_string() == "Folder".to_string() {
                img![attrs!{At::Src => format!["/pkg/obj/folder.svg"]}]
                } else {
                img![attrs!{At::Src => format!["/pkg/obj/file.svg"]}]
                }
                ],
                th![
                    if t.ftype.to_string() == "Folder".to_string() {
                a![
                    format!["{}/", &t.name.to_string()],
                    attrs! {At::Href => format!["{}/",t.name.to_string()]}
                ]
                } else {
                a![
                    format!["{}", &t.name.to_string()],
                    attrs! {At::Href => format!["{}",t.name.to_string()]}
                ]
                }
                ],
                th![
                    &t.ftype
                ]
        ])
        ]
    ]]
}
