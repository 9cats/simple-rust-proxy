use hyper::{service::make_service_fn, service::service_fn, Body, Request, Response, Server};
use std::convert::Infallible;
// use reqwest::Url;
// use hyper::http::uri::InvalidUri;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let target_url = &req.uri().to_string()[1..];  // 从第二个字符开始取

    if !target_url.starts_with("http") {
        return Ok(Response::builder()
            .status(400)
            .body(Body::from("Invalid URL"))
            .unwrap());
    }

    let client = reqwest::Client::new();
    match client.get(target_url).send().await {
        Ok(resp) => {
            let body = match resp.bytes().await {
                Ok(b) => b,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(500)
                        .body(Body::from("Error reading response"))
                        .unwrap())
                }
            };
            Ok(Response::new(Body::from(body)))
        }
        Err(_) => Ok(Response::builder().status(500).body(Body::from("Error occurred")).unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        let func = service_fn(handle_request);
        async move { Ok::<_, Infallible>(func) }
    });

    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
