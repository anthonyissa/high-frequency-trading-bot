use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceApiResponse {
    pub symbol: String,
    pub price: String,
}
