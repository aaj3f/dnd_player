use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::data::character::Character;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayObject {
    pub character: Character,
    #[serde(deserialize_with = "de_created_at", default = "empty_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "de_updated_at", default = "empty_updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_deserializing, default = "empty_datetime")]
    pub last_played_at: DateTime<Utc>,
}

fn empty_datetime() -> DateTime<Utc> {
    Utc::now()
}

fn empty_updated_at() -> Option<DateTime<Utc>> {
    None
}

fn de_created_at<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match Utc.datetime_from_str(&s, "%a %b %e %T %Y") {
        Ok(utc) => Ok(utc),
        Err(_) => Ok(Utc::now()),
    }
}

fn de_updated_at<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match Utc.datetime_from_str(&s, "%a %b %e %T %Y") {
        Ok(utc) => Ok(Some(utc)),
        Err(_) => Ok(None),
    }
}
