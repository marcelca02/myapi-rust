use std::io::Error;

pub fn get_path(req: String) -> Result<String, Error> {
    let first_line = req.split("\r\n").nth(0).unwrap();
    let path = first_line.split(" ").nth(1).unwrap();
    match path.chars().nth(0) {
        Some('/') => {
            if path.len() > 1 {
                Ok(path.to_string())
            } else {
                Err(Error::new(std::io::ErrorKind::Other, "empty path"))
            }
        },
        _ => Err(Error::new(std::io::ErrorKind::Other, "invalid path"))
    }
}
