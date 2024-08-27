use redis::{Client, Commands, Connection, RedisResult};

fn get_connection() -> RedisResult<Connection> {
    let client = Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

pub fn set(key: &String, value: &String) -> RedisResult<()> {
    let mut con = get_connection()?;
    con.set(key, value)
}

pub fn get(key: &String) -> RedisResult<String> {
    let mut con = get_connection()?;

    con.get(key)
}

pub fn incr(key: &String) -> RedisResult<String> {
    let mut con = get_connection()?;

    con.incr(key, 1)
}

pub fn decr(key: &String) -> RedisResult<String> {
    let mut con = get_connection()?;

    con.decr(key, 1)
}
