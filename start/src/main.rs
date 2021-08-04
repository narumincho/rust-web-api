#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::V6(std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::LOCALHOST,
        3000,
        0,
        0,
    ));

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(handle_request))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    println!("development server start! http://{}", addr.to_string());

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(
    request: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    let path = request.uri().path();
    println!("request path: {}", path);
    if path == "/program" {
        match program_response().await {
            Some(js_code) => {
                let mut r = hyper::Response::new(hyper::Body::from(js_code));
                r.headers_mut().insert(
                    http::header::CONTENT_TYPE,
                    http::HeaderValue::from_static("text/javascript"),
                );
                return Ok(r);
            }
            None => {}
        };
    }
    if path == "/wasm" {
        match std::fs::read("./pkg/rust_web_api_bg.wasm") {
            Ok(wasm) => {
                let mut r = hyper::Response::new(hyper::Body::from(wasm));
                r.headers_mut().insert(
                    http::header::CONTENT_TYPE,
                    http::HeaderValue::from_static("application/wasm"),
                );
                return Ok(r);
            }
            Err(_) => {}
        }
    }
    let mut r = hyper::Response::new(hyper::Body::from(
        "<!DOCTYPE html>
    <html lang=\"ja\">
    
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Document</title>
        <script type=\"module\" src=\"/program\">
        </script>
    </head>
    
    <body>
      init
    </body>
    
    </html>",
    ));
    r.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("text/html"),
    );
    Ok(r)
}

async fn program_response() -> Option<String> {
    let output = std::process::Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("no-modules")
        .output();
    match output {
        Ok(ok) => {
            println!("status: {:?}", ok.status);
            println!("stdout: {:?}", std::str::from_utf8(&ok.stdout));
            println!("stderr: {:?}", std::str::from_utf8(&ok.stderr));

            match std::fs::read_to_string("./pkg/rust_web_api.js") {
                Ok(js_code) => {
                    let added_js_code = js_code
                        + "
                    wasm_bindgen(\"wasm\")
                    ";
                    Some(added_js_code)
                }
                Err(error) => {
                    eprintln!("js のファイルを読み取れなかった: {}", error);
                    None
                }
            }
        }
        Err(err) => {
            eprintln!("command error: {}", err);
            None
        }
    }
}
