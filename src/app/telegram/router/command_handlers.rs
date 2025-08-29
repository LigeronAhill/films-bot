use anyhow::Result;
use teloxide::{
    prelude::*, sugar::bot::BotMessagesExt, types::InputFile, utils::command::BotCommands,
};

use crate::app::telegram::{Command, MyDialogue, State, TextCommand};

const START_STICKER: &str =
    "CAACAgIAAxkBAAEPPgForZX41qsn-O4_n0a-DwyMLC1D5wAC2BEAAo-jyEu9EaUKcvRilDYE";

pub async fn start_command_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    bot.delete(&msg).await?;
    dialogue.update(State::Start).await?;
    let sticker = InputFile::file_id(START_STICKER.into());
    bot.send_sticker(msg.chat.id, sticker)
        .reply_markup(TextCommand::keyboard())
        .await?;
    Ok(())
}
pub async fn help_command_handler(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
pub async fn cancel_command_handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    dialogue.exit().await?;
    bot.send_message(msg.chat.id, "CANCELED").await?;
    Ok(())
}
