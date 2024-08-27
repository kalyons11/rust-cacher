use redis::{Client, Commands, RedisResult};

pub fn set(key: &String, value: &String) -> RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    con.set(key, value)
}

pub fn get(key: &String) -> RedisResult<String> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    con.get(key)
}
