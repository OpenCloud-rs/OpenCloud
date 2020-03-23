pub mod component {
    use seed::{*, prelude::*};
    use crate::Msg;

    pub fn folder_list(content: Vec<String>) -> Node<Msg>{
        div![
               content.iter().map(|t| h4![t.to_string()]),
        ]
    }

}