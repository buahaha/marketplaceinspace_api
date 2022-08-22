use rocket::serde::{Serialize, Deserialize};

pub type Stream = Vec<StreamBeat>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct StreamBeat {
    #[serde(rename = "type")]
    pub type_field: String,
    pub time: String,
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
    pub closeout_bid: String,
    pub closeout_ask: String,
    pub status: String,
    pub tradeable: bool,
    pub instrument: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub price: String,
    pub liquidity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    pub price: String,
    pub liquidity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    #[serde(rename = "type")]
    pub type_field: String,
    pub time: String,
}