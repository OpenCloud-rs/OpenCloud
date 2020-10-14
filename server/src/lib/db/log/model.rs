pub enum action_type {
    Delete,
    Upload,
    Get,
}

impl action_type {
    pub fn format(&self) -> String {
        match self {
            action_type::Delete => {String::from("Delete")}
            action_type::Upload => {String::from("Upload")}
            action_type::Get => {String::from("Get")}
        }
    }
}