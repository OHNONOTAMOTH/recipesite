use redis::*;
use std::fs::File;
use std::io::{Write, Error};

pub async fn submittosearch(
    mut con: Connection,
    title: &'static str,
    contents: &'static str,
) -> Result<(), Error> {
    let _: Result<(), RedisError> = redis::cmd("ft.sugadd")
        .arg("recipes")
        .arg(title)
        .query(&mut con);
    
        //let _: () = con.set(title, path).unwrap();

    let mut file = File::create(title.to_owned() + ".md").unwrap();

    file.write_all(&contents.as_bytes())
}

