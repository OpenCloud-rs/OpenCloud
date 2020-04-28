use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonStruct {
	pub result : bool,
	pub lenght : i64,
	pub ftype: FType,
	pub content : Vec<Folder>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
	pub result: bool,
	pub name: String,
	pub ftype: FType,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum FType {
	File,
	Folder,
	Error
}
