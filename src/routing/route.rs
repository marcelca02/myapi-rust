// Route structure for handling routes in the server api

// use regex::Regex;

#[allow(dead_code)]
pub struct Route {
    uri: String,
    action: Box<dyn Fn()>,
    parameters: Vec<String>,
    // regex: Regex,
    // middlewares: Vec<Box<dyn Fn()>>,
}

#[allow(dead_code)]
impl Route {
    pub fn new(uri: &str, action: Box<dyn Fn()>) -> Self {
        Route {
            uri: uri.to_string(),
            action,
            parameters: Vec::new(),
            // regex: Regex::new(uri).unwrap(),
            // middlewares: Vec::new(),
        }
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_action(&self) -> &Box<dyn Fn()> {
        &self.action
    }

    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }
}
