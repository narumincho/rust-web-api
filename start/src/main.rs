fn main() {
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
                    start_js_server(&added_js_code);
                }
                Err(error) => {
                    eprintln!("js のファイルを読み取れなかった: {}", error);
                }
            }
        }
        Err(err) => {
            eprintln!("command error: {}", err)
        }
    }
}

#[tokio::main]
async fn start_js_server(code: &String) {
    // We'll bind to 127.0.0.1:3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    {
        // A `Service` is needed for every connection, so this
        // creates one from our `hello_world` function.
        let make_svc = hyper::service::make_service_fn(|_conn| async {
            // service_fn converts our function into a `Service`
            Ok::<_, std::convert::Infallible>(hyper::service::service_fn(|param| {
                // code にしたい...
                hello_world(param, code)
            }))
        });

        let server = hyper::Server::bind(&addr).serve(make_svc);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

async fn hello_world(
    _req: hyper::Request<hyper::Body>,
    st: &String,
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    Ok(hyper::Response::new(hyper::Body::from(st.clone())))
}
