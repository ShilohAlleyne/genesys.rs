use inquire::error::InquireResult;

#[tokio::main]
async fn main() -> InquireResult<()> {
    genesys::go().await
}
