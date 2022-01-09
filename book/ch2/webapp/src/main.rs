use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("http_requests_total", "HTTP requests total"),
        &["method", "path"]
    ).expect("Can't create a metric");
}

fn main() {
    println!("Starting app..");

    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), Some(labels));

    let server = HttpServer::new(|| {
        return App::new()
                    .wrap(prometheus.clone())
                    .route("/", web::get().to(get_index))
                    .route("/send", web::post().to(post_message))
    });

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000").expect("error binding server to port 3000")
        .run().expect("error starting http server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Hello World</title>
                <body>
                    <form action="/send" method="post">
                        <input type="text" name="n"/>
                        <button type="submit">Send</button>
                    </form>
                </body>
            "#,
        )
}

fn post_message(form: web::Form<MyMessage>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello {}", form.n))
}

pub async fn metrics() -> Result<HttpResponse, CustomError> {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder
        .encode(&prometheus::gather(), &mut buffer)
        .expect("Failed to encode metrics");

    let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
    buffer.clear();

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN))
        .body(response))
}

#[derive(Deserialize)]
struct MyMessage {
    n: String
}
