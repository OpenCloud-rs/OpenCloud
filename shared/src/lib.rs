use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonStruct {
    pub result: bool,
    pub lenght: i64,
    pub ftype: FType,
    pub content: Vec<Folder>,
}
impl Default for JsonStruct {
    fn default() -> Self {
        Self {
            result: false,
            lenght: 0,
            ftype: FType::Error,
            content: Vec::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Folder {
    pub result: bool,
    pub name: String,
    pub size: u64,
    pub created: String,
    pub modified: String,
    pub ftype: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FType {
    File,
    Folder,
    Error,
}

#[derive(Deserialize)]
pub struct HTTPQuery {
    pub rtype: String,
    pub download: String,
}
