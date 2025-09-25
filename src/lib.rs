use inquire::error::InquireResult;

mod cards;
mod data;
mod error;
mod prompt;

pub async fn go() -> InquireResult<()> {
    let new_banlist = data::get_banlist()
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Banlist"))?;
    let saved_banlist = data::load_previous_banlist();

    match saved_banlist {
        Ok(cards) => {
            let cards = data::calculate_delta(new_banlist, cards);
            let selected_cards = prompt::promt(cards)?;

            data::open_card_db(selected_cards)?;

            Ok(())
        }
        Err(e) => {
            eprint!("{}", e);
            Ok(())
        }
    }
}
