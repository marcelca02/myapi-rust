extern crate myapi_rust;
use myapi_rust::App;

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
        res.set_body(body.into())
    });

    app.get("/json", |_req, res| {
        let json_message = r#"{"message": "Hello World"}"#;
        res.set_json_body(json_message.into())
    });

    app.run().await;
}
