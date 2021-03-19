use crate::lib::file::get_size_dir;
use shared::Folder;
use std::fs::Metadata;
pub trait TraitFolder {
    fn from_metadata(e: Metadata, path: String) -> Folder;
    fn error(error: String) -> Folder;
}

impl TraitFolder for Folder {
    fn from_metadata(e: Metadata, path: String) -> Folder {
        let ftype;
        let size;
        if e.is_dir() {
            ftype = "Folder".to_string();
            size = get_size_dir(path.clone())
        } else {
            ftype = mime_guess::from_path(path.split('/').last().unwrap())
                .first_or_octet_stream()
                .to_string();
            size = e.len()
        };
        Folder {
            result: true,
            size,
            created: time::PrimitiveDateTime::from(
                e.created().unwrap_or_else(|_| std::time::SystemTime::now()),
            )
            .format("%d-%m-%Y %T"),
            name: String::from(path.trim_end_matches('/').split('/').last().unwrap()),
            ftype,
            modified: time::PrimitiveDateTime::from(
                e.modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now()),
            )
            .format("%d-%m-%Y %T"),
        }
    }
    fn error(error: String) -> Folder {
        Folder {
            result: false,
            size: 0,
            created: String::from("0-0-0000 00:00:00"),
            modified: String::from("0-0-0000 00:00:00"),
            name: error,
            ftype: String::from("Error"),
        }
    }
}
