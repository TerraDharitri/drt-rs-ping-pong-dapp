use super::request;
use crate::config::Config;
use serde_json::{json, Value};

pub enum TransactionType {
    Ping,
    Pong,
    PongAll,
}

pub async fn ping(config: &Config) -> Result<Value, Value> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{transaction_url}/ping");

    let body = json!({
        "sender": "drt1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssey5egf",
        "value": 0.001
    });

    request::post_request("ping", &endpoint, Some(&body)).await
}

pub async fn sc_setup(config: &Config) -> Result<Value, Value> {
    let setup_url = &config.setup_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{setup_url}");

    let body = json!(
        {
            "ping_amount": 0.001,
            "max_funds": 100,
            "activation_timestamp": "None",
            "duration": 60,
            "deployer": "drt1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssey5egf"

    });

    request::post_request("sc_setup", &endpoint, Some(&body)).await
}

pub async fn pong(config: &Config) -> Result<Value, Value> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{transaction_url}/pong");

    request::post_request("pong", &endpoint, None).await
}

pub async fn pong_all(config: &Config) -> Result<Value, Value> {
    let transaction_url = &config.transaction_url;
    let dest = &config.dest;
    let endpoint = format!("http://{dest}{transaction_url}/pong_all");

    request::post_request("pong_all", &endpoint, None).await
}
