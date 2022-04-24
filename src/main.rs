use redis::*;

pub mod search;
pub mod submit;

#[tokio::main]
async fn main() {
    let cli = Client::open("redis://127.0.0.1").unwrap();
    let mut con = cli.get_connection().unwrap();

    //submit::submittosearch(con, "test1", "test1");
    /*println!(
        "{:?}",
        redis::cmd("get")
            .arg("test")
            .query::<String>(&mut con)
            .unwrap()
    );*/
    println!("{:?}", search::search("test", con).await.unwrap());
}
