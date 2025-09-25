use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    hash::Hash,
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use crate::{cards, error};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    data: Vec<cards::Card>,
}
// --- Data manip ---
pub(crate) fn calculate_delta(
    new_cards: Vec<cards::Card>,
    old_cards: Vec<cards::MinimalCard>,
) -> Vec<cards::Card> {
    let delta = |old: u8, new: u8| -> i16 { new as i16 - old as i16 };
    let old_map: HashMap<u32, cards::MinimalCard> = old_cards.into_iter().map(|c| (c.id, c)).collect();
    let new_map: HashMap<u32, cards::Card> = new_cards.into_iter().map(|c| (c.id, c)).collect();

    let cards_with_delta: Vec<cards::Card> = new_map
        .into_iter()
        .map(|(id, mut card)| {
            let old = old_map
                .get(&id)
                .map(|old| old.genesys_points)
                .unwrap_or(card.genesys_points);

            card.change = delta(old, card.genesys_points);

            card
        })
        .sorted_by(|a, b| a.genesys_points.cmp(&b.genesys_points))
        .collect();

    cards_with_delta
}

pub(crate) fn lens<F, R>(cards: &[cards::Card], accessor: F) -> Vec<R>
where
    F: Fn(&cards::Card) -> R,
    R: Clone + Hash + Eq + Ord,
{
    cards.iter().map(accessor).unique().sorted().collect::<Vec<R>>()
}

// --- YGO Pro API ---
pub(crate) async fn get_banlist() -> Result<Vec<cards::Card>, error::Error> {
    let body =
        reqwest::get("https://db.ygoprodeck.com/api/v7/cardinfo.php?format=genesys&misc=yes")
            .await?
            .text()
            .await?;

    let data: Data = serde_json::from_str(&body).map_err(|e| error::Error::Deserialization {
        path: PathBuf::from_str("Api").unwrap_or_default(),
        source: e,
    })?;

    let cards: Vec<cards::Card> = data
        .data
        .into_iter()
        .filter(|c| c.genesys_points > 0)
        .collect();

    Ok(cards)
}

// --- load/save banlists ---
fn get_path() -> PathBuf {
    let home: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path: PathBuf = PathBuf::from(format!("{}/.genesys/banlist.json", home));

    path
}

pub(crate) fn load_previous_banlist() -> Result<Vec<cards::MinimalCard>, error::Error> {
    let path: PathBuf = get_path();

    if !path.exists() {
        return Ok(Vec::new());
    }

    let json = fs::read_to_string(&path).map_err(|e| error::Error::FileRead {
        path: path.clone(),
        source: e,
    })?;

    let cards = serde_json::from_str(&json).map_err(|e| error::Error::Deserialization {
        path: path.clone(),
        source: e,
    })?;

    Ok(cards)
}

pub(crate) fn save_banlist(cards: Vec<cards::Card>) -> std::io::Result<()> {
    let path = get_path();

    let json = serde_json::to_string_pretty(&cards).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid banlist format")
    })?;

    if !path.exists() {
        // create the opts dir
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    file.write_all(json.as_bytes())?;

    Ok(())
}

// --- Open cards in web browser ---
pub(crate) fn open_card_db(cards: Vec<cards::Card>) -> std::io::Result<()> {
    cards
        .into_iter()
        .filter(|c| c.ygoprodeck_url.is_some())
        .for_each(|c| {
            if webbrowser::open(&c.ygoprodeck_url.unwrap()).is_ok() {
                println!("Opened: {}", c.name);
            }
        });

    Ok(())
}
