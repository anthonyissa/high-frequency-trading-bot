mod computation;
mod finnhub;
mod get_price;
mod trade;
mod types;
mod utils;
use std::thread::sleep;

use computation::buy_if_conditions_met;
use computation::try_closing_past_trades;
use finnhub::get_indicator_single_value;
use finnhub::request_finnhub;
use get_price::get_price;
use trade::buy;
extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    buy_if_conditions_met().await;
    try_closing_past_trades().await;
    sleep(std::time::Duration::from_secs(60));
}
