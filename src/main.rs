use crate::data::{calculate_delta, get_banlist, load_previous_banlist, save_banlist};

mod cards;
mod data;
mod error;

#[tokio::main]
async fn main() ->std::io::Result<()> {
    let new_banlist = get_banlist().await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Options"))?;
    let saved_banlist = load_previous_banlist();

    match saved_banlist {
        Ok(cards) => {
            let cards = calculate_delta(new_banlist, cards);

            for c in cards.iter() {
                println!("{}", c);
            }
        
            Ok(())
        }
        Err(e) => Ok(eprint!("{}", e))
    }
}
