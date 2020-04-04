pub mod component {
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


}

pub mod uploadfile {
    use crate::Msg;
    use seed::{prelude::*, *};
    use std::fmt;
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum State {
        Show,
        Hidden
    }
    impl State {
        pub fn next(self) -> Self {
            match self {
                Self::Show => Self::Hidden,
                Self::Hidden => Self::Show
            }
        }
    }

    impl Default for State {
        fn default() -> Self {
            Self::Hidden
        }
    }
    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::Show => "hidden",
            Self::Hidden => "visible",
        };
        write!(f, "{}", state)
    }}
    pub fn upload_file(state: State, url: &String) -> Node<Msg> {
        div![
                button![format!["{}", match state {
                    self::State::Show => {
                        "Show to upload"
                    }
                    self::State::Hidden => {
                        "Hidden the upload menu"
                    }
                }], ev(Ev::Click,|_| Msg::Next)],
                div![attrs!{At::from("style") => format!["visibility: {}", state]},

                form![
                  attrs!{
                      At::from("method") => "post",
                      At::from("enctype") => "multipart/form-data",
                      At::from("action") => format!["http://127.0.0.1:8080/cli/{}", url],
                  },
                    input![
                        attrs!{
                            At::from("name") => "file",
                            At::from("type") => "file",
                            At::from("accept") => "image/png, image/jpeg"
                        }
                    ],
                    input![
                        attrs!{
                            At::from("type") => "submit",
                        }
                    ]

                ]
            ]
        ]
    }
}
