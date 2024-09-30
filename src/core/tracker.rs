use crate::core::types::{CryptoPair, OkxMessage};
use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use reqwest::blocking::Client;
use serde::Deserialize;
use tokio::signal;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info, instrument, warn};
use url::Url;

#[instrument(skip(pair))]
pub async fn ticker(pair: &CryptoPair) -> Result<()> {
    info!(
        "Attempting to track price for {} on OKX. Press Ctrl+C to stop.",
        pair.to_string()
    );

    let symbol = format!("{}-{}", pair.base.to_uppercase(), pair.quote.to_uppercase());
    let ws_url = "wss://ws.okx.com:8443/ws/v5/public";
    let url = Url::parse(ws_url).context("Failed to parse WebSocket URL")?;

    info!("Connecting to OKX WebSocket at {}", ws_url);

    let (ws_stream, response) = connect_async(url)
        .await
        .context("Failed to connect to WebSocket")?;

    info!("WebSocket connection response: {:?}", response);

    info!(
        "WebSocket connection established. Response status: {}",
        response.status()
    );

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to the ticker channel
    let subscribe_message = serde_json::json!({
        "op": "subscribe",
        "args": [{
            "channel": "tickers",
            "instId": symbol
        }]
    });
    write
        .send(Message::Text(subscribe_message.to_string()))
        .await
        .context("Failed to send subscription message")?;

    info!("Subscription message sent. Waiting for messages...");

    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let message: OkxMessage = serde_json::from_str(&text)
                            .context("Failed to parse WebSocket message as JSON")?;

                        match message {
                            OkxMessage::Subscription(subscription) => {
                                info!("Subscription response: {:?}", subscription);
                            }
                            OkxMessage::Ticker(ticker) => {
                                if !ticker.data.is_empty() {
                                    let ticker = &ticker.data[0];
                                    println!("Current price of {}: ${}", pair.to_string(), ticker.last);
                                    info!("24h High: ${}, 24h Low: ${}, 24h Volume: {}",
                                          ticker.high24h, ticker.low24h, ticker.vol24h);
                                    info!("Ask: ${} ({}), Bid: ${} ({})",
                                          ticker.ask_px, ticker.ask_sz, ticker.bid_px, ticker.bid_sz);
                                } else {
                                    warn!("Received message with empty data: {}", text);
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Ping(_))) => {
                        write.send(Message::Pong(vec![])).await
                            .context("Failed to send Pong response")?;
                        info!("Received Ping, sent Pong");
                    }
                    Some(Err(e)) => {
                        error!("Error receiving message: {}", e);
                        break;
                    }
                    None => {
                        info!("WebSocket stream ended");
                        break;
                    }
                    _ => {}
                }
            }
            _ = signal::ctrl_c() => {
                info!("Received Ctrl+C, shutting down.");
                break;
            }
        }
    }

    Ok(())
}
