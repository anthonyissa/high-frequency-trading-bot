mod polygon;
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
    let response =
        request_polygon("https://api.polygon.io/v2/aggs/ticker/X:BTCUSD/prev?adjusted=true")
            .await
            .unwrap();
    println!("{}", response);
}
