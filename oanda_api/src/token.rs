use std::env;

pub fn token() -> String {
    let token = env::var("OANDA_DEMO_KEY").unwrap();
    token
}