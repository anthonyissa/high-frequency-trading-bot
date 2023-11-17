mod computation;
mod finnhub;
mod get_price;
mod trade;
mod types;
mod utils;
use std::thread::sleep;

use computation::buy_if_conditions_met;
use computation::try_closing_past_trades;
extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    loop {
        buy_if_conditions_met().await;
        try_closing_past_trades().await;
        sleep(std::time::Duration::from_secs(60));
    }
}
