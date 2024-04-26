extern crate myapi_rust;
use myapi_rust::App;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut app = App::new("127.0.0.1", 4221);

    app.get("/hello", |_req, res| {
        println!("Hello World");
        res
    });

    app.post("/epa", |_req, res| {
        println!("Goodbye World");
        res
    });

    app.get("/bye", |_req, res| {
        let body = "Goodbye World".to_string();
        res.send(&body)
    });

    app.get("/json", |_req, res| {
        let json_message = r#"{"message": "Hello World"}"#;
        res.json(json_message)
    });

    app.get("/html", |_req, res| {
        let params = HashMap::new();
        res.render_template("example.html", params)
    });

    app.run().await;
}
