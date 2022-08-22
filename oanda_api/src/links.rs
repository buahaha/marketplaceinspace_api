/// Get demo Oanda api url
/// 
/// # Examples
/// 
/// ```
/// use oanda_api::links::demo;
/// let url = demo();
/// assert_eq!(url, "https://api-fxpractice.oanda.com");
/// ```
pub fn demo() -> &'static str {
    "https://api-fxpractice.oanda.com"
}

/// Get live Oanda api url
/// 
/// # Examples
/// 
/// ```
/// use oanda_api::links::live;
/// let url = live();
/// assert_eq!(url, "https://api-fxtrade.oanda.com");
/// ```
pub fn live() -> &'static str {
    "https://api-fxtrade.oanda.com"
}

/// Get Oanda api demo streaming url
/// 
/// # Examples
/// 
/// ```
/// use oanda_api::links::demo_stream;
/// let url = demo_stream();
/// assert_eq!(url, "https://stream-fxpractice.oanda.com");
/// ```
pub fn demo_stream() -> &'static str {
    "https://stream-fxpractice.oanda.com"
}

/// Get Oanda api live streaming url
/// 
/// # Examples
/// 
/// ```
/// use oanda_api::links::live_stream;
/// let url = live_stream();
/// assert_eq!(url, "https://stream-fxtrade.oanda.com");
/// ```
pub fn live_stream() -> &'static str {
    "https://stream-fxtrade.oanda.com"
}

mod tests {
    use rocket::{tokio};
    use crate::{account::Accounts, token, stream::StreamBeat};
    use std::env;
    use super::*;
    extern crate dotenv;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_demo_url() {
        dotenv().ok();
        let demo_url = demo();
        let request_url = format!("{}/v3/accounts", demo_url);
        let get_accounts = reqwest::Client::new();
        let result = get_accounts
            .get(request_url)
            .bearer_auth(token::token()).send().await;
        let response = result.unwrap();
        match response.status() {
            reqwest::StatusCode::OK => {
                let json = response.json::<Accounts>().await;
                let json_body = json.unwrap();
                let account_1 = &json_body.accounts[0].id;
                let account_2 = &json_body.accounts[1].id;
                let primary_account = env::var("PRIMARY_ACCOUNT").unwrap();
                let secondary_account = env::var("SECONDARY_ACCOUNT").unwrap();
                if account_1 == &primary_account || account_1 == &secondary_account {
                } else {
                    panic!("Account 1 is not the primary or secondary account");
                }
                if account_2 == &primary_account || account_2 == &secondary_account {
                } else {
                    panic!("Account 2 is not the primary or secondary account");
                }
            },
            _ => {
                panic!("Error: {}", response.status());
            }
        }
    }
    
    use rocket::futures::StreamExt;
    use rocket::serde::json;
    use crate::stream::{Stream, Heartbeat};
    extern crate chrono;
    use chrono::*;
    #[tokio::test]
    async fn test_demo__stream_url() {
        let mut beat = false;
        let mut price_tick = false;
        let demo_stream_url = demo_stream();
        dotenv().ok();
        let demo_url = demo_stream();
        let request_url = format!(
            "{}/v3/accounts/{}/pricing/stream?instruments=EUR_USD%2CUSD_CAD", 
            demo_url, env::var("PRIMARY_ACCOUNT").unwrap());
        let get_stream = reqwest::Client::new();
        let result = get_stream
            .get(request_url).bearer_auth(token::token()).send().await;
        let response = result.unwrap();
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item.unwrap();
            let item = String::from_utf8(item.to_vec()).unwrap();
            let item = item.replace("\n", ",");
            let item: &str = &item[0..item.len() - 1];
            eprintln!("{}", item);
            if item.contains("HEARTBEAT") {
                let heartbeat = json::from_str::<Heartbeat>(&item).unwrap();
                eprintln!("{:?}", heartbeat);
                let time = heartbeat.time;
                let time = UTC.datetime_from_str(&time, "%Y-%m-%dT%H:%M:%S.%fZ").unwrap();
                eprintln!("{:?} HEARTBEAT", time);
                beat = true;
            } else {
                let item = format!("[{}]", item);
                let stream = json::from_str::<Stream>(&item).unwrap();
                stream.iter().for_each(|beat| {
                    eprintln!("{:?}", beat);
                    let instrument = &beat.instrument;
                    let bid = &beat.bids[0].price;
                    let ask = &beat.asks[0].price;
                    eprintln!("{}: {} {}", instrument, bid, ask);
                });
                price_tick = true;
            }
            if beat && price_tick {
                break;
            }
        }
    }

    #[tokio::test]
    async fn test_live_urls() {
        let live_url = live();
        let live_stream_url = live_stream();
    }
}