use helldivers2_rs::{models::raw::war::WarId, HellApi};

#[tokio::test]
async fn v1_assignments_endpoint() {
    let result = HellApi::assignments().await;
    let inner = result.unwrap();
    if let Some(first) = inner.first() {
        let result = HellApi::assignment(first.id).await;
        result.unwrap();
    }
}

#[tokio::test]
async fn raw_assignments_endpoint() {
    let result = HellApi::assignments_raw(WarId::from(801)).await;
    result.unwrap();
}
