use crate::{types::Trade, utils::get_current_timestamp};

static mut TRADES: Vec<Trade> = Vec::new();

pub fn buy(ticker: &str, price: f64) {
    println!("Buying {} at {}", ticker, price);
    let trade = Trade {
        ticker: ticker.to_string(),
        price,
        entry_timestamp: get_current_timestamp() as i64,
        exit_timestamp: 0,
        profit: 0.0,
    };
    unsafe {
        TRADES.push(trade);
    }
}

pub fn sell(id: f64, price: f64) {
    unsafe {
        if let Some(trade) = TRADES.get_mut(id as usize) {
            println!("Selling {} at {}", trade.ticker, price);
            trade.exit_timestamp = get_current_timestamp() as i64;
            trade.profit = price - trade.price;
        }
    }
}

pub fn get_unclosed_trades() -> Vec<&'static Trade> {
    let mut to_return: Vec<&Trade> = Vec::new();
    unsafe {
        for trade in TRADES.iter() {
            if trade.exit_timestamp == 0 {
                to_return.push(trade.clone());
            }
        }
    }
    to_return
}

pub fn show_trades() {
    unsafe {
        for trade in TRADES.iter() {
            println!("{:?}", trade);
        }
    }
}
