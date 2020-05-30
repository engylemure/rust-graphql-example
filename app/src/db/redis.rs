use crate::utils::env::ENV;

pub fn connect() -> redis::Client {
    redis::Client::open(ENV.redis_url.clone()).expect("Failed to create redis Client")
}
