use std::fs;
use std::fs::File;
use std::path::PathBuf;

use std::io::Write;
pub trait Code {
    fn _write(dir_path: &str, file_name: &str, code: &str) {
        match fs::create_dir_all(dir_path) {
            Ok(_) => {
                let mut path_buf = PathBuf::from(dir_path);
                path_buf.push(format!("{}.rs", file_name));
                match path_buf.to_str() {
                    None => {}
                    Some(path) => {
                        let mut file = File::create(path).expect("unable to create file");
                        write!(file, "{}", code).expect("unable to write to file");
                    }
                }
            }
            Err(e) => eprintln!("error creating directory: {}", e),
        }
    }
    fn write(path: &str, file_name: Option<&str>);
}

mod marco;
pub mod payloads;

pub mod restful;
