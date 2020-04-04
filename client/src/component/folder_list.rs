use seed::{prelude::*, *};
use crate::Msg;

pub fn folder_list(content: Vec<String>) -> Node<Msg> {
    div![table![
        tr![td![a!["..", attrs! {At::Href => ".."}]]],
        content.iter().map(|t| tr![td![a![
            format!["{}/", t.to_string()],
            attrs! {At::Href => format!["{}/",t.to_string()]}
        ]]])
    ]]
}
