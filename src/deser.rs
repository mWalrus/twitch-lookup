use chrono::Duration;
use serde::{de, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn deserialize_millis<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis: i64 = Deserialize::deserialize(data).unwrap_or(0);
    Ok(Duration::milliseconds(millis))
}

pub fn deserialize_seconds<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds: i64 = Deserialize::deserialize(data).unwrap_or(0);
    Ok(Duration::seconds(seconds))
}

pub fn deserialize_minutes<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let minutes: i64 = Deserialize::deserialize(data).unwrap_or(0);
    Ok(Duration::minutes(minutes))
}

pub fn deserialize_date_time<'de, D, S>(data: D) -> Result<S, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
    S::Err: Display,
{
    let s: String = Deserialize::deserialize(data)?;
    S::from_str(&s).map_err(de::Error::custom)
}

pub fn deserialize_stream_status<'de, D>(data: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let status: String = Deserialize::deserialize(data).unwrap_or_else(|_| String::from("offline"));
    if status != "live" {
        return Ok(false);
    }
    Ok(true)
}
