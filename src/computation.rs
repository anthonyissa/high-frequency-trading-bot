use crate::{
    finnhub::get_indicator_single_value,
    get_price::get_price,
    trade::{buy, get_unclosed_trades, sell},
};

static TP: f64 = 0.005;
static SL: f64 = 0.002;

pub async fn buy_if_conditions_met() {
    println!("Computing indicators data...");
    let rsi = get_indicator_single_value("BINANCE:BTCUSDT", "rsi")
        .await
        .unwrap();
    let ema = get_indicator_single_value("BINANCE:BTCUSDT", "ema")
        .await
        .unwrap();
    let obv = get_indicator_single_value("BINANCE:BTCUSDT", "obv")
        .await
        .unwrap();
    let price = get_price("BTCUSDT").await.unwrap();
    println!("RSI {}", rsi);
    println!("EMA {}", ema);
    println!("OBV {}", obv);
    println!("Price {}", price);
    if rsi == 0.0 || ema == 0.0 || obv == 0.0 {
        println!("Avoiding buying because of 0 values");
        return;
    }
    if rsi < 30.0 && price < ema && obv > 0.0 {
        buy("BTC", price)
    }
}

pub async fn try_closing_past_trades() {
    let trades = get_unclosed_trades();
    for (index, trade) in trades.iter().enumerate() {
        let price = get_price("BTCUSDT").await.unwrap();
        let variation = price / trade.price;
        if variation > 1.0 + TP || variation < 1.0 - SL {
            println!("Closing trade with profit {}", price - trade.price);
            sell(index as f64, price)
        }
    }
}
