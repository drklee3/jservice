use jservice_rs::JServiceRequester;

#[tokio::test]
async fn requests_clues() {
    let client = reqwest::Client::new();

    let clues = client.get_clues(|o| o).await.unwrap();
    println!("{:#?}", clues[0]);

    assert!(clues.len() > 0);
}

#[tokio::test]
async fn requests_random_clues() {
    let client = reqwest::Client::new();

    let clues = client.get_random_clues(5).await.unwrap();
    println!("{:#?}", clues[0]);

    assert_eq!(clues.len(), 5);
}

#[tokio::test]
async fn requests_categories() {
    let client = reqwest::Client::new();

    let categories = client.get_categories(5, 0).await.unwrap();
    println!("{:#?}", categories[0]);

    assert_eq!(categories.len(), 5);
}

#[tokio::test]
async fn requests_category() {
    let client = reqwest::Client::new();

    let category = client.get_category(21).await.unwrap();
    println!("{:#?}", category);

    assert_eq!(category.title, "animals");
}
