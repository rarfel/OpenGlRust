use mimetype_detector::{detect_file};

pub fn verify_file_type(path:&str) -> (&'static str, Vec<u8>) {
    let data = &path[3..];
    match detect_file(&data){
        Err(err) => {
            println!("Error: {}", err);
            ("error", b"error".to_vec())
        },
        Ok(path) => (&path.mime()[6..], std::fs::read(&data).unwrap())
    }
}
