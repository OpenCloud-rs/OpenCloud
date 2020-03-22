pub mod component {
    use seed::{*, prelude::*};
    use crate::Msg;

    pub fn test_component() -> Node<Msg> {
        let item_style = style!{
        St::MarginTop => px(10);
        St::FontSize => unit!(1.2, em)
    };

        div![
        ul![
            li![ &item_style, "Item 1", ],
            li![ &item_style, "Item 2", ],
        ]
    ]
    }

}