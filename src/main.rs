use teloxide::prelude::*;

async fn run_bot() {
    // pretty_env_logger::init();
    // log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    }).await;
}


#[tokio::main]
async fn main() {
    run_bot().await;
}
