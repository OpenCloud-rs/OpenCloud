pub mod component {
    use seed::{*, prelude::*};

    use crate::Msg;

    pub fn folder_list(content: Vec<String>) -> Node<Msg> {
        div![
          table![
            content.iter().map(|t| tr![td![a![t.to_string(),attrs!{At::Href => format!["{}",t.to_string()]}]]])
          ]
        ]
    }
}