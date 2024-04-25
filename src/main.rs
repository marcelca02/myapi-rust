extern crate myapi_rust;
use myapi_rust::App;

#[tokio::main]
async fn main() {
    let mut app = App::new("127.0.0.1", 4221);

    app.get("/hello", |_req, res| {
        println!("Hello World");
        res
    });

    app.run().await;
}
