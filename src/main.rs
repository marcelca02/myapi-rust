extern crate myapi_rust;
use myapi_rust::App;


fn main() {
    let mut app = App::new();


    app.get("/", |req, res| {
        res.send("Hello World");
    });


    app.run("127.0.0.1".to_string(), 4221);
}
