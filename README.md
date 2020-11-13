# jService Rust

Async Rust API wrapper for [jService](https://jservice.io/).

Provides a trait for an existing
[Reqwest](https://github.com/seanmonstar/reqwest) client, allowing you to bring
your own client instead of using a built in one.

## Example

```rust
use jservice_rs::JServiceRequester;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let clues = client
        .get_clues(|options| options
            .value(600)
            .category(21)
        )
        .await?;

    println!("{:?}", clues);

    Ok(())
}
```
