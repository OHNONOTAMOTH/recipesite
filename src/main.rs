use redis::*;
use json::*;
use rocket::*;
use rocket::response::content::{Json, Custom};
use rocket::http::ContentType;

pub mod search;
pub mod submit;

const ADDR: &str = "redis://127.0.0.1";

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

//  added CORS
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
  //submit::submittosearch();
  
  //println!("{:?}", search::search("test", con).await.unwrap());\

  //  start server
  rocket::build().attach(CORS).mount("/", routes![servesearchres, submittorest])
}

//  rest api for searching db
#[get("/apisearch/<url>")]
async fn servesearchres(url: &str) -> Json<String> {
  //  creates connection
  let client = redis::Client::open(ADDR).unwrap();
  let con = client.get_connection().unwrap();
  
  //  searches db for input string
  let results = search::search(url, con).await;
  
  //  returns json of results
  response::content::Json(json::stringify(object! {
    results: results.unwrap()
  }))
}

#[get("/submit/<name>/<content>")]
async fn submittorest(name: &str, content: &str) -> Custom<String> {
  //  creates connection
  let client = redis::Client::open(ADDR).unwrap();
  let con = client.get_connection().unwrap();

  submit::submittosearch(con, name, content).await;
  response::content::Custom(ContentType::Text, "done".to_owned())
}
