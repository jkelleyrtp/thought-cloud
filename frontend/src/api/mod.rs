pub async fn get_data() -> Vec<RingEvent> {
    reqwest::get("http://localhost:8080/count")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
