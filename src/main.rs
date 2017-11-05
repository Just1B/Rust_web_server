extern crate iron;
extern crate time;
extern crate router;
extern crate chrono;
extern crate colored;
extern crate rustc_serialize;

use colored::*;
use chrono::Local;
use router::Router;
use iron::prelude::*;
use iron::mime::Mime;
use time::precise_time_ns;
use rustc_serialize::json;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};

#[derive(RustcEncodable)]
struct JsonResponse {
  response : String
}

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        let date = Local::now();

        println!("- Request received at {} and took : {} ms", date.format("%Y-%m-%d %H:%M:%S").to_string().yellow().bold() ,((delta as f64) / 1000000.0).to_string().green().bold() );
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn json_query(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");

    let response = JsonResponse { response: query.to_string() };
    let out = json::encode(&response).unwrap();

    Ok(Response::with((content_type, iron::status::Ok, out)))
}

fn main() {
    println!(" 
 _____ _____ _____ _____    _ _ _ _____ _____    _____ _____ _____ _____ _____ _____ 
| __  |  |  |   __|_   _|  | | | |   __| __  |  |   __|   __| __  |  |  |   __| __  |
|    -|  |  |__   | | |    | | | |   __| __ -|  |__   |   __|    -|  |  |   __|    -|
|__|__|_____|_____| |_|    |_____|_____|_____|  |_____|_____|__|__|___/|_____|__|__|
");

    let mut router = Router::new();
    router.get("/", hello_world, "index");
    router.get("/:query", json_query, "query");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    Iron::new(chain).http("localhost:3000").unwrap();
}