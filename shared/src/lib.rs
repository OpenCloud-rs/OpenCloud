use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub result: bool,
    pub lenght: i64,
    pub content: Vec<String>,
}

pub struct FolderB {
	pub result : bool,
	pub lenght : i64,
	pub ftype : Type,
	pub content : Vec<Vec<(String, Type)>>
}

pub enum Type {
	File,
	Folder
}