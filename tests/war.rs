use helldivers2_rs::{models::raw::war::WarId, HellApi};

#[tokio::test]
async fn v1_war_endpoint() {
    HellApi::war().await.unwrap();
}

#[tokio::test]
async fn raw_war_id_endpoint() {
    HellApi::war_id().await.unwrap();
}

#[tokio::test]
async fn raw_war_status_endpoint() {
    HellApi::war_status(&WarId::from(801)).await.unwrap();
}

#[tokio::test]
async fn raw_war_info_endpoint() {
    HellApi::war_info(&WarId::from(801)).await.unwrap();
}

#[tokio::test]
async fn raw_war_summary_endpoint() {
    HellApi::war_summary(&WarId::from(801)).await.unwrap();
}
