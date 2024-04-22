extern crate myapi_rust;
use myapi_rust::{App, Request, Response};

fn main() {
    let mut app = App::new("127.0.0.1", 4221);
    app.get("/hello", |_req: &Request, _res: &mut Response| {
        Response::empty()
    });

    app.run();
}
