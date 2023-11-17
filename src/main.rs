mod get_price;
mod polygon;
mod types;
use get_price::get_price;
use polygon::request_polygon;
extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let response =
        request_polygon("https://api.polygon.io/v2/aggs/ticker/X:BTCUSD/prev?adjusted=true")
            .await
            .unwrap();
    let price = get_price("BTCUSDT").await.unwrap();

    println!("{}", price);
}
