use inquire::{formatter::MultiOptionFormatter, InquireError, MultiSelect};

use crate::{cards, data};

fn filter_by_points(cards: &mut Vec<cards::Card>, search_point_vals: &[u8]) {
    if search_point_vals.is_empty() {
        return;
    }
    cards.retain(|card| search_point_vals.contains(&card.genesys_points));
}

fn filter_by_card_type(cards: &mut Vec<cards::Card>, search_card_types: &[String]) {
    if search_card_types.is_empty() {
        return;
    }
    cards.retain(|card| search_card_types.contains(&card.card_type));
}

fn filter_by_level(cards: &mut Vec<cards::Card>, search_levels: &[String]) {
    if search_levels.is_empty() {
        return;
    }

    cards.retain(|card| match card.level {
        Some(lvl) => search_levels.contains(&lvl.to_string()),
        None => search_levels.contains(&"None".to_string()),
    });
}

fn filter_by_archetype(cards: &mut Vec<cards::Card>, search_archetypes: &[String]) {
    if search_archetypes.is_empty() {
        return;
    }

    cards.retain(|card| match &card.archetype {
        Some(arc) => search_archetypes.contains(arc),
        None => search_archetypes.contains(&"None".to_string()),
    });
}

pub(crate) fn promt(cards: Vec<cards::Card>) -> Result<Vec<cards::Card>, InquireError> {
    // --- Formatters ---
    let multiselect_formatter: MultiOptionFormatter<String> = &|a| {
        a.iter()
            .map(|item| item.value.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let u8_multiselect_formatter: MultiOptionFormatter<u8> = &|a| {
        a.iter()
            .map(|item| item.value.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let card_formatter: MultiOptionFormatter<cards::Card> = &|a| {
        a.iter()
            .map(|item| item.value.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let mut pool = cards.clone();

    // --- Step 1: Point Values ---
    let point_choices = data::lens(&pool, |c| c.genesys_points);
    let point_val = MultiSelect::new("Select point value(s):", point_choices)
        .with_formatter(u8_multiselect_formatter)
        .prompt()?;

    filter_by_points(&mut pool, &point_val);

    // --- Step 2: Card Types ---
    let type_choices = data::lens(&pool, |c| c.card_type.clone());
    let card_type = MultiSelect::new("Select card type(s):", type_choices)
        .with_formatter(multiselect_formatter)
        .prompt()?;

    filter_by_card_type(&mut pool, &card_type);

    // --- Step 3: Archetypes ---
    let archetype_choices = data::lens(&pool, |c| {
        c.archetype.clone().unwrap_or_else(|| "None".to_string())
    });
    let archetype = MultiSelect::new("Select archetype(s):", archetype_choices)
        .with_formatter(multiselect_formatter)
        .prompt()?;

    filter_by_archetype(&mut pool, &archetype);

    // --- Step 4: Levels ---
    let level_choices = data::lens(&pool, |c| {
        c.level.map_or("None".to_string(), |lvl| lvl.to_string())
    });
    let level = MultiSelect::new("Select level(s):", level_choices)
        .with_formatter(multiselect_formatter)
        .prompt()?;

    filter_by_level(&mut pool, &level);

    // --- Final Selection ---
    let selected_cards = MultiSelect::new("Select card(s):", pool)
        .with_formatter(card_formatter)
        .prompt()?;

    if selected_cards.is_empty() {
        return Err(InquireError::Custom(
            "You must select at least one card.".into(),
        ));
    }

    data::save_banlist(cards)?;

    Ok(selected_cards)
}
