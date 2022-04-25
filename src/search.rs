use redis::*;

pub async fn search(title: &str, mut con: Connection) -> Result<Vec<String>, RedisError> {
    let result = redis::cmd("ft.sugget")
        .arg("recipesearch")
        .arg(title)
        .query(&mut con);

    Ok(result.unwrap())
}
