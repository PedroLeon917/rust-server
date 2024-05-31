use std::net::IpAddr;
use warp::{Filter, http::Response};

#[tokio::main]
async fn main() {
    let api = warp::path!("api" / "name" / String)
        .and(warp::get())
        .map(|name| {
            let s = format!("Hello {}!", name);
            println!("Got request, answering with: {}", s);
            Response::builder().body(s)
        });
    let ip_addr: IpAddr = "0.0.0.0".parse().unwrap();
    warp::serve(api).run((ip_addr, 8000)).await;
}
// Query with
// curl http://localhost:8000/api/name/peter