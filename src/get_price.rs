use crate::{types::CoinbaseApiResponse, utils::get_current_timestamp};

static mut PRICES: Vec<f64> = Vec::new(); // 14 length -> 14 minutes
static mut last_price_update_timestamp: i64 = 0;

pub fn format_response(response: String) -> f64 {
    let coinbase_response: CoinbaseApiResponse = serde_json::from_str(&response).unwrap();
    let price = coinbase_response.data.amount.parse::<f64>().unwrap();
    return price;
}

pub async fn get_price(ticker: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coinbase.com/v2/prices/{}/buy".replace("{}", ticker);
    let response: String = reqwest::get(url).await?.text().await?;
    let price: f64 = format_response(response);
    if get_current_timestamp() as i64 - unsafe { last_price_update_timestamp } < 60 {
        update_last_price(price);
        return Ok(price);
    }
    push_price(price);
    unsafe { last_price_update_timestamp = get_current_timestamp() as i64 };
    Ok(price)
}

pub fn update_last_price(price: f64) {
    unsafe {
        PRICES.pop();
        PRICES.push(price);
    }
}

pub fn push_price(price: f64) {
    unsafe {
        PRICES.push(price);
        if PRICES.len() > 14 {
            PRICES.remove(0);
        }
    }
}

pub fn get_prices() -> Vec<f64> {
    unsafe { PRICES.clone() }
}
