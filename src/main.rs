mod finnhub;
mod get_price;
mod trade;
mod types;
mod utils;
use finnhub::get_rsi;
use finnhub::request_finnhub;
use get_price::get_price;
use trade::buy;
extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let response =
        request_finnhub("https://api.polygon.io/v2/aggs/ticker/X:BTCUSD/prev?adjusted=true")
            .await
            .unwrap();
    let price = get_price("BTCUSDT").await.unwrap();
    get_rsi("BINANCE:BTCUSDT").await.unwrap();
    println!("{}", price);
    buy("BTC", 100.0)
}
