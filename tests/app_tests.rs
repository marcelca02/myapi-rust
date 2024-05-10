
#[cfg(test)]
mod response_test {
    extern crate myapi_rust;
    
    use myapi_rust::{App, Response};
    use reqwest;

    // Response tests

    #[test]
    fn response_send() {
        let mut res = Response::empty();
        res.send("Hello, World!");
        let body = res.get_body().unwrap();
        assert_eq!(body, "Hello, World!".as_bytes());
    }

    #[test]
    fn response_json() {
        let mut res = Response::empty();
        res.json(r#"{"message": "Hello, World!"}"#);
        let body = res.get_body().unwrap();
        // The Content-Type shoud be setted to application/json too
        assert_eq!(body, r#"{"message": "Hello, World!"}"#.as_bytes());
    }

    #[test]
    #[ignore]
    fn response_render_template() {
        let mut res = Response::empty();
        let mut params = std::collections::HashMap::new();
        params.insert("name".to_string(), "John".to_string());
        res.render_template("test.html", params);
        // The Content-Type shoud be setted to text/html too
        let body = res.get_body().unwrap();
        assert_eq!(body, "Hello, John!".as_bytes());
    }

    // App tests

    #[tokio::test]
    #[ignore]   // Server can not be stopped
    async fn app_get() {
        let task = tokio::spawn(async {
            let mut app = App::new("127.0.0.1", 8080);
            app.get("/hello", |_req, res| {
                res.send("Hello, World!")
            });
            app.run().await;
        });
        println!("Server started");

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let response = reqwest::get("http://localhost:8080/hello").await.unwrap();
        println!("Status: {}", response.status());
        assert_eq!(response.status(), 200);

        task.abort();
    }

    #[tokio::test]
    #[ignore]
    async fn app_get_not_found() {
        let task = tokio::spawn(async {
            let mut app = App::new("127.0.0.1", 8080);
            app.get("/hello", |_req, res| {
                res.send("Hello, World!")
            });
            app.run().await;
        });
        println!("Server started");

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let response = reqwest::get("http://localhost:8080/bye").await.unwrap();
        assert_eq!(response.status(), 404);

        task.abort();
    }
}
