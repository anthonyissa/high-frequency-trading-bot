use types::BinanceApiResponse;

use crate::types;

pub fn format_response(response: String) -> f64 {
    println!("Response: {}", response);
    let binance_response: BinanceApiResponse = serde_json::from_str(&response).unwrap();
    binance_response.price.parse::<f64>().unwrap()
}

pub async fn get_price(ticker: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api1.binance.com/api/v3/ticker/price?symbol={}".replace("{}", ticker);
    let response: String = reqwest::get(url).await?.text().await?;
    let price: f64 = format_response(response);
    Ok(price)
}
