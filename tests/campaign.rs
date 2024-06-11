use helldivers2_rs::HellApi;

#[tokio::test]
async fn v1_campaigns_endpoint() {
    let result = HellApi::campaigns().await;
    let inner = result.unwrap();
    let first = inner.first().unwrap();

    let result = HellApi::campaign(first.id).await;
    result.unwrap();
}
