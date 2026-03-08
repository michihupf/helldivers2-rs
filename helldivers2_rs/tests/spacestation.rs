use helldivers2_rs::HellApi;

#[tokio::test]
async fn v2_space_station_endpoint() {
    HellApi::init("helldivers2-rs", "");
    let space_stations = HellApi::space_stations().await.unwrap();
    let first = space_stations.first().unwrap();
    HellApi::space_station(first.id).await.unwrap();
}
