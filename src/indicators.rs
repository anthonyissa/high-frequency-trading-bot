use crate::get_price::get_prices;

pub fn get_rsi() -> f64 {
    if get_prices().len() < 14 {
        return 0.0;
    }
    let prices = get_prices();
    let mut gains: Vec<f64> = Vec::new();
    let mut losses: Vec<f64> = Vec::new();

    for (index, price) in prices.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let variation = price / prices[index - 1];
        if variation > 1.0 {
            gains.push(variation - 1.0);
        } else {
            losses.push(1.0 - variation);
        }
    }

    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;
    for gain in gains.iter() {
        avg_gain += gain;
    }
    for loss in losses.iter() {
        avg_loss += loss;
    }
    avg_gain /= gains.len() as f64;
    avg_loss /= losses.len() as f64;

    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));
    rsi
}
