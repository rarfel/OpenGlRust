use image::{load, ImageResult, DynamicImage};
use mimetype_detector::{detect_file};

fn verify_file_type(path:&str, tries:u8) -> (&'static str, Vec<u8>) {
    let tried = tries - 1;
    let data = &path;
    match detect_file(&data){
        Err(_) => {
            if tried <= 0 {
                return ("ERROR", b"error".to_vec());
            }
            verify_file_type(&path[3..], tried)
        },
        // '.mime()' return 'image/type' slicing 'image/' of
        Ok(path) => (&path.mime()[6..], std::fs::read(&data).unwrap())
    }
}

pub fn load_image(path:&str) -> ImageResult<DynamicImage>{
    let (mime, file_content) = verify_file_type(&path, 2);
    match mime{
        "png"=>load(std::io::Cursor::new(&file_content), image::ImageFormat::Png),
        "jpeg"=>load(std::io::Cursor::new(&file_content), image::ImageFormat::Jpeg),
        "gif"=>load(std::io::Cursor::new(&file_content), image::ImageFormat::Gif),
        "ERROR" => panic!("{}! Verify if path name is correct", mime),
        &_ => panic!("File type \"{}\" not supported right now", mime),
    }
}
