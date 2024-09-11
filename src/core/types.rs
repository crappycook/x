#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct CryptoPair {
    pub base: String,
    pub quote: String,
}

impl CryptoPair {
    pub fn new(base: &str, quote: &str) -> Self {
        CryptoPair {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OkxMessage {
    Subscription(OkxSubscriptionResponse),
    Ticker(OkxWebSocketMessage),
}

#[derive(Deserialize, Debug)]
pub struct OkxWebSocketMessage {
    pub arg: OkxArg,
    pub data: Vec<OkxTickerData>,
}

#[derive(Deserialize, Debug)]
pub struct OkxSubscriptionResponse {
    pub event: String,
    pub arg: OkxArg,
    #[serde(rename = "connId")]
    pub conn_id: String,
}

#[derive(Deserialize, Debug)]
pub struct OkxArg {
    pub channel: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
}

#[derive(Deserialize, Debug)]
pub struct OkxTickerData {
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub last: String,
    #[serde(rename = "lastSz")]
    pub last_sz: String,
    #[serde(rename = "askPx")]
    pub ask_px: String,
    #[serde(rename = "askSz")]
    pub ask_sz: String,
    #[serde(rename = "bidPx")]
    pub bid_px: String,
    #[serde(rename = "bidSz")]
    pub bid_sz: String,
    pub open24h: String,
    pub high24h: String,
    pub low24h: String,
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    #[serde(rename = "volCcy24h")]
    pub vol_ccy24h: String,
    pub vol24h: String,
    pub ts: String,
}
