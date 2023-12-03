mod computation;
mod finnhub;
mod get_price;
mod indicators;
mod notification;
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
        match async {
            buy_if_conditions_met().await;
            try_closing_past_trades().await;
            Result::<(), Box<dyn std::error::Error>>::Ok(())
        }
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("An error occurred: {:?}", err);
                continue;
            }
        }
        sleep(std::time::Duration::from_secs(10));
    }
}
