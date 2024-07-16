use std::fs::File;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut f = match File::options().read(true).write(true).open(file) {
        Ok(f) => f,
        Err(_) => match File::create(file){
            Ok(f) => f,
            Err(err) => panic! ("Failed to create file: {}", err),
        },
    };

    match f.write_all(content.as_bytes()){
        Ok(_) => (),
        Err(err) => panic!("Failed to write content to file: {}", err),
    }
}