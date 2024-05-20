use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;

pub fn format_html(file_path: &str, params: HashMap<String, String>) -> Result<String, Error> {
    let file_string = format!("./tests/templates/{}", file_path);
    let file= Path::new(&file_string);

    if !file.exists() {
        return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", file_path)));
    }

    let mut content = fs::read_to_string(file)?;
    let mut old_content = content.clone();
    for (var, val) in params {
        content = content.replace(&format!("{{{{{}}}}}", var), &val);            
        match content == old_content {
            true => {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Variable {} not found in file {}", var, file_path)));
            },
            false => old_content = content.clone(),
        }
    }

    Ok(content)
}

pub fn format_regex(path: &str, params: &mut Vec<String>) -> String {
    let mut regex = String::new();
    regex.push_str("^");
    for (_, part) in path.split('/').enumerate() {
        if part.starts_with(':') {
            regex.push_str("/([^/]+)");
            params.push(part[1..].to_string());
        } else if part != "" {
            regex.push_str("/");
            regex.push_str(part);
        }
    }
    regex
}
