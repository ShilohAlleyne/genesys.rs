use std::fmt::{self, Display};

use colored::Colorize;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    #[serde(skip_serializing)]
    #[serde(rename = "humanReadableCardType")]
    pub card_type: String,
    #[serde(skip_serializing)]
    pub level: Option<u8>,
    #[serde(skip_serializing)]
    pub archetype: Option<String>,
    #[serde(skip_serializing)]
    pub ygoprodeck_url: Option<String>,
    #[serde(rename = "misc_info", deserialize_with = "extract_genesys_points")]
    pub genesys_points: u8,
    #[serde(skip_deserializing, skip_serializing)]
    pub change: i16,
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let change = match self.change {
            n if n < 0 => format!("{}{}", '\u{2193}', n.abs()), // ↓
            n if n > 0 => format!("{}{}", '\u{2191}', n.abs()), // ↑
            _ => "→0".to_string(),
        };

        let points = match self.genesys_points {
            n if n > 75 => format!("{:<3}", n).red(),
            n if n > 50 => format!("{:<3}", n).yellow(),
            n if n > 25 => format!("{:<3}", n).green(),
            n => format!("{:<3}", n).blue(),
        };

        let padded = format!("{:<75}", self.name);
        write!(f, "{} | {} | {}", padded, points, change)
    }
}

fn extract_genesys_points<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::Number(n) => n
            .as_u64()
            .map(|v| v as u8)
            .ok_or_else(|| de::Error::custom("Invalid number for genesys_points")),
        Value::Array(arr) => arr
            .first()
            .and_then(|v| v.get("genesys_points"))
            .and_then(|gp| gp.as_u64())
            .map(|gp| gp as u8)
            .ok_or_else(|| de::Error::custom("Missing genesys_points in misc_info[0]")),
        _ => Err(de::Error::custom("Unexpected format for misc_info")),
    }
}

// Minal card data for calculating change in points
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct MinimalCard {
    pub id: u32,
    pub name: String,
    #[serde(rename = "misc_info", deserialize_with = "extract_genesys_points")]
    pub genesys_points: u8,
}
