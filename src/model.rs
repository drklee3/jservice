use chrono::naive::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u64,

    pub title: String,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: NaiveDateTime,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: NaiveDateTime,

    pub clues_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct Clue {
    pub id: u64,

    pub answer: String,

    pub question: String,

    pub value: Option<u64>,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub airdate: NaiveDateTime,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: NaiveDateTime,

    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: NaiveDateTime,

    pub category_id: u64,

    pub game_id: Option<u64>,

    pub invalid_count: Option<u64>,

    pub category: Category,
}

#[derive(Debug, Default, Serialize)]
pub struct ClueOptions {
    pub value: Option<u64>,

    pub category: Option<u64>,

    #[serde(serialize_with = "serialize_datetime")]
    pub min_date: Option<NaiveDateTime>,

    #[serde(serialize_with = "serialize_datetime")]
    pub max_date: Option<NaiveDateTime>,

    pub offset: Option<u64>,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ").map_err(de::Error::custom)
}

fn serialize_datetime<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = date {
        let s = format!("{}", date.format("%Y-%m-%dT%H:%M:%S.%fZ"));
        serializer.serialize_str(&s)
    } else {
        serializer.serialize_none()
    }
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
    pub fn min_date(&mut self, min_date: NaiveDateTime) -> &mut Self {
        self.0.min_date.replace(min_date);
        self
    }

    /// Latest date to show, based on original air date
    pub fn max_date(&mut self, max_date: NaiveDateTime) -> &mut Self {
        self.0.max_date.replace(max_date);
        self
    }

    /// Offsets the returned clues. Useful in pagination
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.0.offset.replace(offset);
        self
    }
}
