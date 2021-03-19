use crate::Msg;
use seed::{prelude::*, *};

pub fn breadcrumb(url: String) -> Node<Msg> {
    let mut n = 0;
    let mut path = String::new();
    let mut node: Vec<Node<Msg>> = Vec::new();
    for t in url.split('/').collect::<Vec<&str>>() {
        if !t.is_empty() || t != "/" {
            path.push_str(format!("{}/", t).as_str());
        }
        let updatepath = path.clone();
        let e: Node<Msg> = if t.is_empty() {
                    if n == 0 {
                        li![""]
                    } else {
                        empty![]
                    }
                } else {
                    n+= 1;
                    li![
                        a![
                            t.to_string(),
                            ev(Ev::Click, move |_| {log!(&updatepath); Msg::UpdatePath(format!("{}/", updatepath))})
                        ]
                    ]
                };

                node.push(e);
    };
    nav![
        C!["breadcrumb is-centered bd-snippet notification has-text-black has-background-link-light"],
            ul![
                li![
                    format!{""}
                ],
                node.as_slice()
                ]
        ]
}

pub fn get_to(num: usize, vec: &Vec<&str>) -> String {
    let mut result = String::new();

    for i in vec.iter().enumerate() {
        if i.0 == num {
            break;
        } else {
            result.push_str(i.1.clone());
        }
    }

    result

}