use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonStruct {
    pub result: bool,
    pub lenght: i64,
    pub content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonStructB {
	pub result : bool,
	pub lenght : i64,
	pub rtype: FType,
	pub content : Vec<Folder>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
	pub result: bool,
	pub name: String,
	pub ftype: FType,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum FType {
	File,
	Folder,
	Error
}
