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

    app.get("/hey/:name/:lastname", |req, res| {
        let name = req.get_param("name").unwrap();
        let lastname = req.get_param("lastname").unwrap();
        let body = format!("Buenas {} {}", name, lastname);
        res.send(&body)
    });

    app.get("/buenas/:name", |req, res| {
        let name = req.get_param("name").unwrap();
        let body = format!("Buenas {}", name);
        res.send(&body)
    });


    let hola = app.router("hola");

    hola.get("/mundo", |_req, res| {
        let body = "Hola Mundo hola".to_string();
        res.send(&body)
    });

    hola.get("/caracola", |_req, res| {
        let body = "Hola Caracola hola".to_string();
        res.send(&body)
    });

    hola.post("/caracola", |_req, res| {
        println!("Goodbye Caracola hola");
        res
    });

    // La ruta que se ejecuta es la de app porque tiene el router default
    app.get("/mundo", |_req, res| {
        let body = "Hola mundo app".to_string();
        res.send(&body)
    });

    app.get("/json", |_req, res| {
        let json_message = r#"{"message": "Hello World"}"#;
        res.json(json_message)
    });

    app.get("/foto", |_req, res| {
        res.send_file("image.png")
    });

    app.get("/pdf", |_req, res| {
        res.send_file("Avaluacio_de_projectes.pdf")
    });

    app.get("/txt", |_req, res| {
        res.send_file("hola.txt")
    });

    app.get("/html", |_req, res| {
        let params = HashMap::new();
        res.render_template("example.html", params)
    });

    app.run().await;
}
