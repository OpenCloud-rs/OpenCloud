
    use std::fs;

    pub fn dir_content(path: &str) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        match fs::read_dir(path) {
            Ok(_f) => {
                for path in _f {
                    vec.push(path.unwrap().path().display().to_string());
                }
            }
            Err(_e) => {
                vec.push(String::from("Error"));
                println!("Le dossier est inexistant");
            }
        };
        vec
    }
