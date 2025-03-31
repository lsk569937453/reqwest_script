use hmac::{Hmac, KeyInit, Mac};
use reqwest::header;
use rust_decimal::prelude::*;
use serde_json::Value;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
//use hex_literal::hex;

type HmacSha256 = Hmac<Sha256>;

fn get_timestamp(time: SystemTime) -> u128 {
    time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn get_signature(request: String, secret_key: &str) -> String {
    let mut signed_key = HmacSha256::new_from_slice(secret_key.as_bytes())
        .expect("HMAC could not be created from slice");
    signed_key.update(request.as_bytes());
    hex::encode(signed_key.finalize().into_bytes())
}

pub fn get_binancetest_balance() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    // 从环境变量获取API密钥和密钥
    let api_key = "hhQqQqttwlDf8aNirQnokDNR4CUusER1msgtFZeZDduZatGRiKZa54UJN2XOgEiJ";
    let secret_key = "bAEsJUVYFJPqhfkPnZhtBaHo529bkCdcNGkVi5DmKvZa9wQI6wmubuvRl75OZTvn";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        header::HeaderName::from_static("x-mbx-apikey"),
        header::HeaderValue::from_str(api_key)?,
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    // 获取时间戳和签名
    let timestamp = get_timestamp(SystemTime::now());
    let params = format!("timestamp={}", timestamp);
    let signature = get_signature(params.clone(), secret_key);

    // 构建完整请求URL
    // let request_url = format!(
    //     "https://testnet.binance.vision/api/v3/account?{}&signature={}",
    //     params, signature
    // );
    // 构建完整请求URL
    let request_url = format!(
        "https://api.binance.com/api/v3/account?{}&signature={}",
        params, signature
    );

    // 发送请求并处理响应
    let response = client.get(&request_url).send()?;
    let result: Value = response.json()?;
    // println!("{}", result);
    // 提取余额数据
    let balances = result["balances"]
        .as_array()
        .ok_or("Failed to parse balances")?;

    Ok(balances.clone())
}
