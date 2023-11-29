pub async fn send_notification(text: &str) {
    reqwest::get(
        std::env::var("NOTIFICATION_URL")
            .unwrap()
            .replace("{}", text)
            .to_string(),
    )
    .await;
}
