use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u64,

    pub title: String,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,

    pub clues_count: u64,

    pub clues: Option<Vec<Clue>>,
}

#[derive(Debug, Deserialize)]
pub struct Clue {
    pub id: u64,

    pub answer: String,

    pub question: String,

    pub value: Option<u64>,

    pub airdate: DateTime<Utc>,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,

    pub category_id: u64,

    pub game_id: Option<u64>,

    pub invalid_count: Option<u64>,

    pub category: Option<Category>,
}

#[derive(Debug, Default, Serialize)]
pub struct ClueOptions {
    pub value: Option<u64>,

    pub category: Option<u64>,

    pub min_date: Option<DateTime<Utc>>,

    pub max_date: Option<DateTime<Utc>>,

    pub offset: Option<u64>,
}

#[derive(Default)]
pub struct ClueOptionsBuilder(pub ClueOptions);

impl ClueOptionsBuilder {
    /// Value of the clue in dollars
    pub fn value(&mut self, value: u64) -> &mut Self {
        self.0.value.replace(value);
        self
    }

    /// Id of the category you want to return
    pub fn category(&mut self, category: u64) -> &mut Self {
        self.0.category.replace(category);
        self
    }

    /// Earliest date to show, based on original air date
    pub fn min_date(&mut self, min_date: DateTime<Utc>) -> &mut Self {
        self.0.min_date.replace(min_date);
        self
    }

    /// Latest date to show, based on original air date
    pub fn max_date(&mut self, max_date: DateTime<Utc>) -> &mut Self {
        self.0.max_date.replace(max_date);
        self
    }

    /// Offsets the returned clues. Useful in pagination
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.0.offset.replace(offset);
        self
    }
}
