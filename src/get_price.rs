use crate::types::{self, CoinbaseApiResponse};

static mut PRICES: Vec<f64> = Vec::new(); // 14 length -> 14 minutes

pub fn format_response(response: String) -> f64 {
    let coinbase_response: CoinbaseApiResponse = serde_json::from_str(&response).unwrap();
    let price = coinbase_response.data.amount.parse::<f64>().unwrap();
    return price;
}

pub async fn get_price(ticker: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coinbase.com/v2/prices/{}/buy".replace("{}", ticker);
    let response: String = reqwest::get(url).await?.text().await?;
    let price: f64 = format_response(response);
    push_price(price);
    Ok(price)
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
