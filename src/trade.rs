use crate::{types::Trade, utils::get_current_timestamp};

static mut TRADES: Vec<Trade> = Vec::new();
static mut ID: f64 = 0.0;

pub fn buy(ticker: &str, price: f64) {
    println!("**** Buying {} at {} ****", ticker, price);
    let trade = Trade {
        id: unsafe {
            ID += 1.0;
            ID
        },
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
        let trade = TRADES.iter_mut().find(|trade| trade.id == id).unwrap();
        println!("**** Selling {} at {} ****", trade.ticker, price);
        trade.exit_timestamp = get_current_timestamp() as i64;
        trade.profit = price - trade.price;
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

pub fn show_stats() -> (f64, usize, usize) {
    unsafe {
        let mut total_profit = 0.0;
        let mut total_trades = 0;
        let mut total_open_trades = 0;
        for trade in TRADES.iter() {
            if trade.exit_timestamp == 0 {
                total_open_trades += 1;
            }
            if trade.exit_timestamp != 0 {
                total_profit += trade.profit;
                total_trades += 1;
            }
        }
        println!("Total profit {}", total_profit);
        println!("Total open trades {}", total_open_trades);
        println!("Total trades {}", total_trades);
        return (total_profit, total_open_trades, total_trades);
    }
}
