use bytes::Bytes;
use serde_derive::{Serialize, Deserialize};
use std::net::IpAddr;
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiDoc {
    name: String,
    des: String,
}

#[tokio::main]
async fn main() {
    let api_doc = ApiDoc {
        name: String::from("API Name"),
        des: String::from("API Description"),
    };
    
    let root_route = 
    warp::path::end()
        .map( || {
            "Wellcome to PROS Marketplace!"
        });

    let doc_route = 
    warp::path!("api")
        .map(move || {
            warp::reply::json(&api_doc)
        });

    let api_route = 
    warp::path!("api" / "person")
        .and(warp::post())
        .and(warp::body::bytes())
        .map(|bodybytes: Bytes| {
            let p : Person = serde_json::from_slice(&bodybytes[..]).unwrap();
            println!("Got Person: {:?}", p);
            warp::reply::json(&p)
        });
    
    let routes = 
    root_route.or(api_route).or(doc_route);

    let ip_addr: IpAddr = "0.0.0.0".parse().unwrap();
    warp::serve(routes).run((ip_addr, 8000)).await;
}
// Query with
// curl http://localhost:8000/api/person -d '{"name":"Max", "age": 54}' -v