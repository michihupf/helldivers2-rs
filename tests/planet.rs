use helldivers2_rs::HellApi;

#[tokio::test]
async fn v1_planets_endpoint() {
    HellApi::init("helldivers2-rs", "");
    let result = HellApi::planets().await;
    let inner = result.unwrap();
    let first = inner.first().unwrap();

    let result = HellApi::planet(first.id).await;
    result.unwrap();
}

#[tokio::test]
async fn v1_planet_events_endpoint() {
    HellApi::init("helldivers2-rs", "");
    let result = HellApi::planet_events().await;
    result.unwrap();
}
