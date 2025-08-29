use anyhow::Result;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, InputFile, KeyboardRemove, ParseMode},
};
use tracing::instrument;

use crate::app::{
    storage::Storage,
    telegram::{MyCallback, MyDialogue, State, TextCommand},
    tmdb::Tmdb,
};
const BACK_STICKER: &str =
    "CAACAgIAAxkBAAEPRV9osZ-0Phhpaqp1o508hNxXSdFLbgAC7BUAAukAARhItE_tlWzTa_g2BA";

#[instrument(name = "cancel callback", skip_all)]
pub async fn cancel_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    let _ = cb;
    if let Some(msg) = q.regular_message() {
        let sticker = InputFile::file_id(BACK_STICKER.into());
        bot.send_sticker(msg.chat.id, sticker)
            .reply_markup(TextCommand::keyboard())
            .await?;
    }
    Ok(())
}
#[instrument(name = "get film details callback", skip_all)]
pub async fn get_film_details_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::GetFilmsDetails { id } = cb {
            let film = tmdb_client.get_films_details(id).await?;
            let file = tmdb_client.get_image(&film.poster_path).await?;
            let text = film.to_string();
            let mu = InlineKeyboardMarkup::default()
                .append_row(vec![
                    MyCallback::AddFilmToWatchList { id: film.id }.into(),
                    MyCallback::GetFilmsCredits { id: film.id }.into(),
                ])
                .append_row(vec![MyCallback::Cancel.into()]);
            bot.send_photo(msg.chat.id, file)
                .caption(text)
                .parse_mode(ParseMode::Html)
                .reply_markup(mu)
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "get film credits callback", skip_all)]
pub async fn get_film_credits_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::GetFilmsCredits { id } = cb {
            let credits = tmdb_client.get_films_credits(id).await?;
            let text = credits.to_string();
            let mu = InlineKeyboardMarkup::default()
                .append_row(vec![
                    MyCallback::AddFilmToWatchList { id }.into(),
                    MyCallback::GetFilmsDetails { id }.into(),
                ])
                .append_row(vec![MyCallback::Cancel.into()]);
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(mu)
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "add film to watch list", skip_all)]
pub async fn add_film_to_watch_list_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    storage: Storage,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    let user_id = q.from.id.0;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::AddFilmToWatchList { id } = cb {
            storage.add_film_to_watch_list(user_id, id).await?;
            let film = tmdb_client.get_films_details(id).await?;
            bot.send_message(
                msg.chat.id,
                format!("Фильм:\n{film}\n Добавлен в список для просмотра"),
            )
            .reply_markup(TextCommand::keyboard())
            .parse_mode(ParseMode::Html)
            .await?;
        }
    }
    Ok(())
}
#[instrument(name = "mark film watched", skip_all)]
pub async fn mark_film_watched_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    storage: Storage,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    let user_id = q.from.id.0;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::MarkFilmWatched { id } = cb {
            storage.watch_film(user_id, id).await?;
            bot.send_message(msg.chat.id, "Оцените фильм по 10-ти бальной шкале")
                .reply_markup(KeyboardRemove::new())
                .await?;
            dialogue
                .update(State::FilmRateReceived { film_id: id })
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "rate film", skip_all)]
pub async fn rate_film_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::RateFilm { id } = cb {
            bot.send_message(msg.chat.id, "Оцените фильм по 10-ти бальной шкале")
                .reply_markup(KeyboardRemove::new())
                .await?;
            dialogue
                .update(State::FilmRateReceived { film_id: id })
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "mark film unwatched", skip_all)]
pub async fn mark_film_unwatched_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    storage: Storage,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    let user_id = q.from.id.0;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::MarkFilmUnWatched { id } = cb {
            storage.unwatch_film(user_id, id).await?;
            bot.send_message(msg.chat.id, "Фильм отмечен непросмотренным")
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "delete film from list", skip_all)]
pub async fn delete_film_from_list_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    storage: Storage,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    let user_id = q.from.id.0;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::DeleteFilm { id } = cb {
            storage.delete_film_from_watch_list(user_id, id).await?;
            bot.send_message(msg.chat.id, "Фильм удален из списка")
                .await?;
        }
    }
    Ok(())
}
