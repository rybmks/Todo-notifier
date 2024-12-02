use std::fmt::Error;
use todo::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    bot_start(Bot::from_env()).await;

    Ok(())
}
