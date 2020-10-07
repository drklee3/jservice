mod error;
pub mod model;

use async_trait::async_trait;
pub use error::Result;
use model::{Category, Clue, ClueOptionsBuilder};
use reqwest::Client;

macro_rules! api {
    ($e:expr) => {
        concat!("https://jservice.io/api", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

#[async_trait]
pub trait JServiceRequester {
    async fn get_clues<F>(&self, f: F) -> Result<Vec<Clue>>
    where
        F: FnOnce(&mut ClueOptionsBuilder) -> &mut ClueOptionsBuilder + Send;

    async fn get_random_clues(&self, count: u64) -> Result<Vec<Clue>>;
    async fn get_categories(&self, count: u64, offset: u64) -> Result<Vec<Category>>;
    async fn get_category(&self, category_id: u64) -> Result<Category>;
    async fn mark_clue_invalid(&self, clue_id: u64) -> Result<()>;
}

#[async_trait]
impl JServiceRequester for Client {
    async fn get_clues<F>(&self, f: F) -> Result<Vec<Clue>>
    where
        F: FnOnce(&mut ClueOptionsBuilder) -> &mut ClueOptionsBuilder + Send,
    {
        let mut create_options = ClueOptionsBuilder::default();
        let options = f(&mut create_options);

        self.get(api!("/clues"))
            .query(&options.0)
            .send()
            .await?
            .json::<Vec<Clue>>()
            .await
            .map_err(Into::into)
    }

    async fn get_random_clues(&self, count: u64) -> Result<Vec<Clue>> {
        self.get(api!("/random"))
            .query(&[("count", count)])
            .send()
            .await?
            .json::<Vec<Clue>>()
            .await
            .map_err(Into::into)
    }

    async fn get_categories(&self, count: u64, offset: u64) -> Result<Vec<Category>> {
        self.get(api!("/categories"))
            .query(&[("count", count), ("offset", offset)])
            .send()
            .await?
            .json::<Vec<Category>>()
            .await
            .map_err(Into::into)
    }

    async fn get_category(&self, category_id: u64) -> Result<Category> {
        self.get(api!("/category"))
            .query(&[("id", category_id)])
            .send()
            .await?
            .json::<Category>()
            .await
            .map_err(Into::into)
    }

    async fn mark_clue_invalid(&self, clue_id: u64) -> Result<()> {
        self.get(api!("/invalid"))
            .query(&[("id", clue_id)])
            .send()
            .await?;

        Ok(())
    }
}
