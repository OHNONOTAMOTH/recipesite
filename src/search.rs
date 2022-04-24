use redis::*;

pub fn search(title: &str, mut con: Connection) -> Result<String, RedisError> {
    let result = redis::cmd("ft.sugget")
        .arg("recipesearch")
        .arg(title)
        .query(&mut con);

    Ok(result)
}
