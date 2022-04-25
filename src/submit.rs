use redis::*;
use std::fs::File;
use std::io::{Write, Error};

pub async fn submittosearch(
    mut con: Connection,
    title: &str,
    contents: &str,
) -> Result<(), Error> {
    //  create database entry
    let _: Result<(), RedisError> = redis::cmd("ft.sugadd")
        .arg("recipesearch")
        .arg(title)
        .arg("1")
        .query(&mut con);
    
        //let _: () = con.set(title, path).unwrap();
    //  create file
    let mut file = File::create("files/".to_owned() + title + ".md").unwrap();

    //  write to file
    file.write_all(&str::replace(&str::replace(&contents, "<", "&#60"), ">", "&#62").as_bytes())
}
