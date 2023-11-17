use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref KEYS: Vec<String> = {
        dotenv().ok();
        env::var("POLYGON_KEYS")
            .unwrap_or_default()
            .split(' ')
            .map(String::from)
            .collect()
    };
}
static mut N: i32 = 0;

fn get_key() -> &'static str {
    let key = &KEYS[unsafe { N as usize }];
    unsafe {
        N += 1;
        if N == KEYS.len() as i32 {
            N = 0;
        }
    }
    key
}

pub async fn request_polygon(url: &str) -> Result<String, reqwest::Error> {
    let key = get_key();
    let resp = reqwest::get(url.to_owned() + "&apiKey=" + key).await?;
    let text = resp.text().await?;
    Ok(text)
}
