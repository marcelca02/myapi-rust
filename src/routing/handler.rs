
use crate::http::comm::{Request, Response};

pub trait RouteHandler: Fn(&Request, &mut Response) -> Response {
    fn call(&self, req: &Request, res: &mut Response) -> Response;
}

impl RouteHandler for Box<dyn Fn(&Request, &mut Response) -> Response> {
    fn call(&self, req: &Request, res: &mut Response) -> Response {
        self(req, res)
    }
}

