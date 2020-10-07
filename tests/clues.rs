use jservice_rs::JServiceRequester;

#[tokio::test]
async fn requests_clues() {
    let client = reqwest::Client::new();

    let clues = client.get_clues(|o| o).await.unwrap();
    println!("{:?}", clues[0]);

    assert!(clues.len() > 0);
}

#[tokio::test]
async fn requests_random_clues() {
    let client = reqwest::Client::new();

    let clues = client.get_random_clues(5).await.unwrap();
    println!("{:?}", clues[0]);

    assert_eq!(clues.len(), 5);
}
