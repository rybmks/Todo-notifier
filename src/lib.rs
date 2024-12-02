pub mod comands;
pub mod create_todo_dialog;

pub async fn bot_start(bot: Bot) {
    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => help_mes(bot, msg).await,
        Command::CreateNewTodo => create_todo_dialog::get_todo_info(bot).await,
    };

    Ok(())
}

async fn help_mes(bot: Bot, msg: Message) {
    let r = bot
        .send_message(msg.chat.id, Command::descriptions().to_string())
        .await;
    println!("{:?}", r);
}

use comands::Command;
use teloxide::prelude::*;
use teloxide::repls::CommandReplExt;
use teloxide::utils::command::BotCommands;
pub use teloxide::Bot;
