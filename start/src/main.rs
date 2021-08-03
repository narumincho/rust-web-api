use rand::prelude::*;

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let random_value: u8 = rng.gen();
    let random_value_as_text: String = random_value.to_string();

    // We'll bind to 127.0.0.1:3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = hyper::service::make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(|_request| {
            hello_world(_request, "やあ".to_string())
        }))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(
    _request: hyper::Request<hyper::Body>,
    text: String,
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    Ok(hyper::Response::new(hyper::Body::from(text)))
}
