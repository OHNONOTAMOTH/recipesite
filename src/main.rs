use redis::*;
use json::*;
use rocket::*;
use rocket::response::content::Json;

pub mod search;
pub mod submit;

const ADDR: &str = "redis://127.0.0.1";

#[launch]
async fn rocket() -> _ {
  //submit::submittosearch();
  
  //println!("{:?}", search::search("test", con).await.unwrap());\
  
  //  start server
  rocket::build().mount("/", routes![route])
}

//  rest api for searching db
#[get("/search/<url>")]
async fn route(url: &str) -> Json<String> {
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
