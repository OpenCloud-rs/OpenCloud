use crate::ChangeRouteType;
use crate::Msg;
use seed::{prelude::*, *};
use shared::Folder;

pub fn folder_list(mut content: Vec<Folder>, url: String) -> Node<Msg> {
    content.sort();
    let mut folder_list = vec![];
    for t in content {
        let name = t.clone().name;
        let path = format!("{}{}", url.clone(), name.clone());
        folder_list.push(tr![
            th![if t.ftype.to_string() == "Folder".to_string() {
                img![attrs! {At::Src => format!["/pkg/obj/folder.svg"]}]
            } else {
                img![attrs! {At::Src => format!["/pkg/obj/file.svg"]}]
            }],
            th![if t.ftype.to_string() == "Folder".to_string() {
                a![
                    format!["{}/", &t.name.to_string()],
                    ev(Ev::Click, move |_| Msg::ChangeRoute(
                        format!("{}", name),
                        ChangeRouteType::Add
                    ))
                ]
            } else {
                a![
                    format!["{}", &t.name.to_string()],
                    attrs! {At::Href => format!["{}",t.name.to_string()]}
                ]
            }],
            th![&t.ftype],
            th![button![
                C!["button is-link"],
                "Delete",
                ev(Ev::Click, move |_| Msg::CallDelete(path))
            ],]
        ])
    }
    div![table![
        C!["table is-hoverable is-fullwidth"],
        thead![tr![th![""], th!["Name"], th!["Type"],],],
        tbody![
            tr![
                th![],
                th![a![
                    "..",
                    ev(Ev::Click, move |_| Msg::ChangeRoute(
                        "".to_string(),
                        ChangeRouteType::Remove
                    ))
                ]],
                th!["Folder"],
            ],
            folder_list,
        ]
    ]]
}
