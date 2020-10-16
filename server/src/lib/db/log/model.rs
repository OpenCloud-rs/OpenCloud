pub enum ActionType {
    Delete,
    Upload,
    Get,
}

impl ActionType {
    pub fn format(&self) -> String {
        match self {
            ActionType::Delete => {String::from("Delete")}
            ActionType::Upload => {String::from("Upload")}
            ActionType::Get => {String::from("Get")}
        }
    }
}