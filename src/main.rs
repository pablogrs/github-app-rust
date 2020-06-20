
use futures::{future, Future, Stream};
use hyper::{
    client::HttpConnector, rt, service::service_fn, Body, Client, Request,
    Response, Server, Method, StatusCode, header
};

use serde_json::{Value};


fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:7878".parse().unwrap();
    type GenericError = Box<dyn std::error::Error + Send + Sync>;
    type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = GenericError> + Send>;
    
    
    fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => four_oh_four(),
            (&Method::POST, "/ping") => process_request(req),
            _ => unimplemented!(),
            }
            
    }

    static NOTFOUND: &[u8] = b"Oops! Not Found";

    fn four_oh_four() -> ResponseFuture {
        let body = Body::from(NOTFOUND);
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap(),
        ))
    }

    
    fn process_request(req: Request<Body>) -> ResponseFuture {
        Box::new(
            req.into_body()
                .concat2() // concatenate all the chunks in the body
                .from_err() // like try! for Result, but for Futures
                .and_then(|whole_body| {
                    let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                    //let words: Vec<&str> = str_body.split('=').collect();
                    redirect_home(str_body)
                }),
        )
    }

    fn redirect_home(request: String) -> ResponseFuture {
        println!("{}", request);

        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(serde_json::json!(request).to_string()))
                .unwrap(),
        ))
    }


    rt::run(future::lazy(move || {
        // create a Client for all Services
        let client = Client::new();

        // define a service containing the router function
        let new_service = move || {
            // Move a clone of Client into the service_fn
            let client = client.clone();
            service_fn(move |req| router(req, &client))
        };

        // Define the server - this is what the future_lazy() we're building will resolve to
        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        println!("Listening on http://{}", addr);
        server
    }));
}

// fn get_payload_request(){

// }

// fn verify_webhook_signature() {

// }

// fn authenticate_app() {

// }

// fn authenticate_installation() {

// }
