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
    
    //println!("{:?}", search::search("test", con).await.unwrap());
    rocket::build().mount("/", routes![route])
}

#[get("/search/<url>")]
async fn route(url: &str) -> Json<String> {
  let client = redis::Client::open(ADDR).unwrap();
  let con = client.get_connection().unwrap();

  let results = search::search(url, con).await;
  response::content::Json(json::stringify(object! {
    results: results.unwrap()
  }))
}
