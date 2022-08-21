
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::serde::{json::Value, Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn ext_jwt() -> String {
    let key_str = env::var("EXANTE_SHARED_KEY").expect(
        "can't get
            Exante shared key from .env",
    );
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(key_str.as_bytes()).expect("can't locksmith the key");
    // let mut claims = Json;
    let iat: i32;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs());
            iat = n.as_secs() as i32;
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    let iss = env::var("EXANTE_CLIENT_ID_ISS").expect(
        "can't get
            Exante client id from .env",
    );
    let sub = env::var("EXANTE_APPLICATION_ID_SUB").expect(
        "can't get
            Exante app id from .env",
    );
    let tokenize = Tokenization {
        iss: iss,
        sub: sub,
        iat: iat,
        exp: iat + 5,
        aud: [
            "symbols".to_string(),
            // leat privilage //
            // "ohlc".to_string(),
            // "feed".to_string(),
            // "change".to_string(),
            // "crossrates".to_string(),
            // "orders".to_string(),
            // "summary".to_string(),
            // "accounts".to_string(),
            // "transactions".to_string(),
        ]
        .to_vec(),
    };
    let token_str = tokenize.sign_with_key(&key).expect("can't sign JWT token");
    token_str
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Tokenization {
    pub iss: String,
    pub sub: String,
    pub iat: i32,
    pub exp: i32,
    pub aud: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct AssetInfo {
    pub option_data: Value,
    pub name: String,
    pub symbol_id: String,
    pub description: String,
    pub icon: String,
    pub underlying_symbol_id: Value,
    pub country: String,
    pub identifiers: Identifiers,
    pub exchange: String,
    pub symbol_type: String,
    pub currency: String,
    pub min_price_increment: String,
    pub ticker: String,
    pub expiration: Value,
    pub group: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Identifiers {
    #[serde(rename = "ISIN")]
    pub isin: String,
    #[serde(rename = "FIGI")]
    pub figi: String,
    #[serde(rename = "RIC")]
    pub ric: String,
    #[serde(rename = "SEDOL")]
    pub sedol: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
