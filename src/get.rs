use redis::*;

pub async fn get(p: &str, mut con: Connection) -> Result<String, RedisError> {
    return redis::cmd("get")
        .arg(p)
        .query(&mut con);
}