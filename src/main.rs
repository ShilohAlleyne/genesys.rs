use inquire::error::InquireResult;

#[tokio::main]
async fn main() -> InquireResult<()> {
    genesys_ygo_cli::go().await
}
