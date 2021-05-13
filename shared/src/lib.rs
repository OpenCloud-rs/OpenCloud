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
            ftype: FType::Unset,
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

impl Folder {
    pub fn new(result: bool, name: String, size: u64, created: String, modified: String, ftype: String) -> Self {
        Self {
            result,
            name,
            size,
            created,
            modified,
            ftype
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FType {
    File,
    Folder,
    Error,
    Unset,
}
