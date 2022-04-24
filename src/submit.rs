use redis::*;
use tokio::*;

#[tokio::main]
pub async fn submittosearch(
    mut con: Connection,
    title: &'static str,
    path: &'static str,
) -> Result<&'static str, &'static str> {
    let _: Result<(), RedisError> = redis::cmd("ft.sugadd")
        .arg("recipes")
        .arg(title)
        .query(&mut con);
    let _: () = con.set(title, path).unwrap();
    Ok("did it")
}
