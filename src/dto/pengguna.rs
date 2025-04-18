use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

fn serialize_naive_datetime_option<S>(
  date: &Option<NaiveDateTime>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match date {
      Some(d) => serializer.serialize_str(&d.format("%Y-%m-%d %H:%M:%S").to_string()),
      None => serializer.serialize_none(),
  }
}

fn deserialize_naive_datetime_option<'de, D>(
  deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let opt = Option::<String>::deserialize(deserializer)?;
  match opt {
      Some(s) => NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
          .map(Some)
          .map_err(serde::de::Error::custom),
      None => Ok(None),
  }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Pengguna {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub level: i32,

    #[serde(
      serialize_with = "serialize_naive_datetime_option",
      deserialize_with = "deserialize_naive_datetime_option",
      skip_serializing_if = "Option::is_none"
  )]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PenggunaStore {
    pub name: String,
    pub username: String,
    pub password: String,
    pub level: i32,
}
