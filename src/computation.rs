use crate::{
    get_price::get_price,
    indicators::get_all_indicators,
    notification::send_notification,
    trade::{buy, get_unclosed_trades, sell, show_stats, show_trades},
};

static TP: f64 = 0.0020;
static SL: f64 = 0.0013;

pub async fn buy_if_conditions_met() {
    if (get_unclosed_trades().len() as f64) > 0.0 {
        return;
    }
    println!("Computing indicators data...");
    // let rsi = get_indicator_single_value("BINANCE:BTCUSDT", "rsi")
    //     .await
    //     .unwrap();
    // let ema = get_indicator_single_value("BINANCE:BTCUSDT", "ema")
    //     .await
    //     .unwrap();
    // let obv = get_indicator_single_value("BINANCE:BTCUSDT", "obv")
    //     .await
    //     .unwrap();
    let price = get_price("BTC-USDT").await.unwrap();
    let (rsi, ema) = get_all_indicators();
    println!("RSI {}", rsi);
    println!("EMA {}", ema);
    // println!("OBV {}", obv);
    println!("Price {}", price);
    if rsi == 0.0 || ema == 0.0 {
        println!("Avoiding buying because of 0 values");
        return;
    }
    if rsi < 30.0 && price < ema {
        buy("BTC", price);
        send_notification("Buying BTC").await;
    }
}

pub async fn try_closing_past_trades() {
    let trades = get_unclosed_trades();
    for (_, trade) in trades.iter().enumerate() {
        let price = get_price("BTC-USDT").await.unwrap();
        let variation = price / trade.price;
        println!("Price {}", price);
        println!("Trade price {}", trade.price);
        println!("Variation {}", variation);
        if variation > 1.0 + TP || variation < 1.0 - SL {
            sell(trade.id, price);
            show_trades();
            let mut msg =
                "Closing trade with profit {}".replace("{}", &(price - trade.price).to_string());
            let stats = show_stats();
            msg.push_str(&format!(
                "%0ATotal profit: {}%0ATotal open trades: {}%0ATotal trades: {}",
                stats.0, stats.1, stats.2
            ));
            println!("{}", msg);
            send_notification(&msg).await;
        }
    }
}
