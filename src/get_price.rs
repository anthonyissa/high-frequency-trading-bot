use crate::types::{self, CoinbaseApiResponse};

pub fn format_response(response: String) -> f64 {
    println!("Response: {}", response);
    let coinbase_response: CoinbaseApiResponse = serde_json::from_str(&response).unwrap();
    let price = coinbase_response.data.amount.parse::<f64>().unwrap();
    return price;
}

pub async fn get_price(ticker: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coinbase.com/v2/prices/{}/buy".replace("{}", ticker);
    let response: String = reqwest::get(url).await?.text().await?;
    let price: f64 = format_response(response);
    Ok(price)
}
