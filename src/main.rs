use teloxide::prelude::*;
mod sheets;

use sheets::append_to_spreadsheet;
use std::env;
use chrono::Utc;


fn get_spreadsheet_id() -> String {
    env::var("SPREADSHEET_ID").unwrap_or_else(|_| {
        panic!("Environment variable SPREADSHEET_ID is not set")
    })
}

// 1tpiEu-Ou2cVi-382xIwEhLu6BAUOVmH8n8co0At1X8w/edit#gid=824503092
async fn run_bot() {
    // pretty_env_logger::init();
    // log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let chat_id = msg.chat.id;
        let username = msg.from().map(|user| user.username.clone().unwrap_or_default())
            .unwrap_or_default();
        let date = chrono::Utc::now().to_rfc3339();

        bot.send_dice(chat_id).await?;

        let sheet_id = get_spreadsheet_id();
        let range = "Sheet1!A1:C1";
        // let values = vec![vec![chat_id.to_string(), username, date]];

        append_to_spreadsheet(&sheet_id, range).await;

        Ok(())
    }).await;
}


#[tokio::main]
async fn main() {
    run_bot().await;

}
