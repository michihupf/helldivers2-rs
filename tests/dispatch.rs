use helldivers2_rs::{models::raw::war::WarId, HellApi};

#[tokio::test]
async fn v1_dispatches_endpoint() {
    let result = HellApi::dispatches().await;
    let inner = result.unwrap();
    let first = inner.first().unwrap();

    let result = HellApi::dispatch(first.id).await;
    result.unwrap();
}

#[tokio::test]
async fn v1_steam_endpoint() {
    let result = HellApi::steam_newsfeed().await;
    let inner = result.unwrap();
    let first = inner.first().unwrap();

    let result = HellApi::steam_newsitem(&first.id).await;
    result.unwrap();
}

#[tokio::test]
async fn raw_news_feed_endpoint() {
    let result = HellApi::news_feed(WarId::from(801)).await;
    result.unwrap();
}
