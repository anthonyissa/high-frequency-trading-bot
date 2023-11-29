use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

use crate::utils::get_current_timestamp;

lazy_static! {
    static ref KEYS: Vec<String> = {
        dotenv().ok();
        env::var("FINNHUB_KEYS")
            .unwrap_or_default()
            .split(' ')
            .map(String::from)
            .collect()
    };
}
static mut N: i32 = 0;

fn get_key() -> &'static str {
    let key = &KEYS[unsafe { N as usize }];
    unsafe {
        N += 1;
        if N == KEYS.len() as i32 {
            N = 0;
        }
    }
    key
}

pub async fn request_finnhub(url: &str) -> Result<String, reqwest::Error> {
    let key = get_key();
    let resp = reqwest::get(url.to_owned() + "&token=" + key).await?;
    let text = resp.text().await?;
    Ok(text)
}

pub async fn get_indicator_single_value(
    ticker: &str,
    indicator: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let current_time = get_current_timestamp();
    let from = &(current_time - 400).to_string();
    let to = &current_time.to_string();
    let url = format!(
        "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution=1&from={}&to={}&indicator={}&limit=%!s(int=0)",
        ticker, from, to, indicator
    );
    let response: String = request_finnhub(&url).await?;
    let values = format_indicator_response(response, indicator);
    Ok(values)
}

pub fn format_indicator_response(response: String, indicator: &str) -> f64 {
    if response.contains("no_data") {
        println!("Error No Data for indicator {}: {}", indicator, response);
        return 0.0;
    }
    let formatted: serde_json::Value = serde_json::from_str(&response).unwrap();
    let lastIndex = formatted[indicator].as_array().unwrap().len() - 1;
    let value = formatted[indicator][lastIndex].as_f64();
    let at = formatted["t"][lastIndex].as_i64();
    return value.unwrap();
}
