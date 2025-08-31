use anyhow::{Context, Result};
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
#[instrument(name = "get serial details callback", skip_all)]
pub async fn get_serial_details_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::GetSerialDetails { id } = cb {
            let tv_show = tmdb_client.get_tv_show_details(id).await?;
            let file = tmdb_client.get_image(&tv_show.poster_path).await?;
            let text = tv_show.to_string();
            let mu = InlineKeyboardMarkup::default()
                .append_row(vec![
                    MyCallback::AddSerialToWatchList { id: tv_show.id }.into(),
                    MyCallback::GetSerialCredits { id: tv_show.id }.into(),
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
#[instrument(name = "get serial credits callback", skip_all)]
pub async fn get_serial_credits_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::GetSerialCredits { id } = cb {
            let credits = tmdb_client.get_tv_show_credits(id).await?;
            let text = credits.to_string();
            let mu = InlineKeyboardMarkup::default()
                .append_row(vec![
                    MyCallback::AddSerialToWatchList { id }.into(),
                    MyCallback::GetSerialDetails { id }.into(),
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
#[instrument(name = "add serial to watch list", skip_all)]
pub async fn add_serial_to_watch_list_callback_handler(
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
        if let MyCallback::AddSerialToWatchList { id } = cb {
            storage.add_serial_to_watch_list(user_id, id).await?;
            let tv_show = tmdb_client.get_tv_show_details(id).await?;
            bot.send_message(
                msg.chat.id,
                format!("Сериал:\n{tv_show}\n Добавлен в список для просмотра"),
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
#[instrument(name = "mark serial watched", skip_all)]
pub async fn mark_serial_watched_callback_handler(
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
        if let MyCallback::MarkSerialWatched { id } = cb {
            storage.watch_serial(user_id, id).await?;
            bot.send_message(msg.chat.id, "Оцените сериал по 10-ти бальной шкале")
                .reply_markup(KeyboardRemove::new())
                .await?;
            dialogue
                .update(State::SerialRateReceived { serial_id: id })
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
#[instrument(name = "mark serial unwatched", skip_all)]
pub async fn mark_serial_unwatched_callback_handler(
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
        if let MyCallback::MarkSerialUnWatched { id } = cb {
            storage.unwatch_serial(user_id, id).await?;
            bot.send_message(msg.chat.id, "Сериал отмечен непросмотренным")
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
#[instrument(name = "rate serial", skip_all)]
pub async fn rate_serial_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    if let Some(msg) = q.regular_message() {
        if let MyCallback::RateSerial { id } = cb {
            bot.send_message(msg.chat.id, "Оцените сериал по 10-ти бальной шкале")
                .reply_markup(KeyboardRemove::new())
                .await?;
            dialogue
                .update(State::SerialRateReceived { serial_id: id })
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
#[instrument(name = "delete serial from list", skip_all)]
pub async fn delete_serial_from_list_callback_handler(
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
        if let MyCallback::DeleteSerial { id } = cb {
            storage.delete_serial_from_watch_list(user_id, id).await?;
            bot.send_message(msg.chat.id, "Сериал удален из списка")
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "search films pagination", skip_all)]
pub async fn search_film_pagination_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    if let Some(msg) = q.regular_message() {
        match cb {
            MyCallback::SearchFilmsNextPage {
                search_string,
                page,
            }
            | MyCallback::SearchFilmsPreviousPage {
                search_string,
                page,
            } => {
                let result = tmdb_client.search_film(search_string.clone(), page).await?;
                let films = result.results;
                let total = result.total_pages;
                if films.is_empty() {
                    bot.send_message(msg.chat.id, "Ничего не найдено")
                        .reply_markup(TextCommand::keyboard())
                        .await?;
                } else {
                    let l = films.len();
                    for (i, film) in films.iter().enumerate() {
                        let text = film.to_string();
                        match film.poster_path.as_ref() {
                            Some(image_url) => {
                                let file = tmdb_client
                                    .get_image(&image_url)
                                    .await
                                    .context("Getting image")?;
                                let mut mu = InlineKeyboardMarkup::default().append_row(vec![
                                    MyCallback::GetFilmsDetails { id: film.id }.into(),
                                    MyCallback::AddFilmToWatchList { id: film.id }.into(),
                                ]);
                                if i == l - 1 {
                                    let mut row = Vec::new();
                                    if page != 1 {
                                        row.push(
                                            MyCallback::SearchFilmsPreviousPage {
                                                search_string: search_string.clone(),
                                                page: page - 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if total as u8 > page {
                                        row.push(
                                            MyCallback::SearchFilmsNextPage {
                                                search_string: search_string.clone(),
                                                page: page + 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if !row.is_empty() {
                                        mu = mu.append_row(row);
                                    }
                                }
                                bot.send_photo(msg.chat.id, file)
                                    .caption(&text)
                                    .parse_mode(ParseMode::Html)
                                    .reply_markup(mu)
                                    .await?;
                            }
                            None => {
                                let mut mu = InlineKeyboardMarkup::default().append_row(vec![
                                    MyCallback::GetFilmsDetails { id: film.id }.into(),
                                    MyCallback::AddFilmToWatchList { id: film.id }.into(),
                                ]);
                                if i == l - 1 {
                                    let mut row = Vec::new();
                                    if page != 1 {
                                        row.push(
                                            MyCallback::SearchFilmsPreviousPage {
                                                search_string: search_string.clone(),
                                                page: page - 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if total as u8 > page {
                                        row.push(
                                            MyCallback::SearchFilmsNextPage {
                                                search_string: search_string.clone(),
                                                page: page + 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if !row.is_empty() {
                                        mu = mu.append_row(row);
                                    }
                                }
                                bot.send_message(msg.chat.id, text)
                                    .parse_mode(ParseMode::Html)
                                    .reply_markup(mu)
                                    .await?;
                            }
                        }
                    }
                    bot.send_message(msg.chat.id, "Вот результаты поиска")
                        .reply_markup(TextCommand::keyboard())
                        .await?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
#[instrument(name = "search serials pagination", skip_all)]
pub async fn search_serial_pagination_callback_handler(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    cb: MyCallback,
    tmdb_client: Tmdb,
) -> Result<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    dialogue.exit().await?;
    if let Some(msg) = q.regular_message() {
        match cb {
            MyCallback::SearchSerialNextPage {
                search_string,
                page,
            }
            | MyCallback::SearchSerialPreviousPage {
                search_string,
                page,
            } => {
                let result = tmdb_client
                    .search_tvshow(search_string.clone(), page)
                    .await?;
                let serials = result.results;
                let total = result.total_pages;
                if serials.is_empty() {
                    bot.send_message(msg.chat.id, "Ничего не найдено")
                        .reply_markup(TextCommand::keyboard())
                        .await?;
                } else {
                    let l = serials.len();
                    for (i, serial) in serials.iter().enumerate() {
                        let text = serial.to_string();
                        match serial.poster_path.as_ref() {
                            Some(image_url) => {
                                let file = tmdb_client
                                    .get_image(&image_url)
                                    .await
                                    .context("Getting image")?;
                                let mut mu = InlineKeyboardMarkup::default().append_row(vec![
                                    MyCallback::GetSerialDetails { id: serial.id }.into(),
                                    MyCallback::AddSerialToWatchList { id: serial.id }.into(),
                                ]);
                                if i == l - 1 {
                                    let mut row = Vec::new();
                                    if page != 1 {
                                        row.push(
                                            MyCallback::SearchSerialPreviousPage {
                                                search_string: search_string.clone(),
                                                page: page - 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if total as u8 > page {
                                        row.push(
                                            MyCallback::SearchSerialNextPage {
                                                search_string: search_string.clone(),
                                                page: page + 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if !row.is_empty() {
                                        mu = mu.append_row(row);
                                    }
                                }
                                bot.send_photo(msg.chat.id, file)
                                    .caption(&text)
                                    .parse_mode(ParseMode::Html)
                                    .reply_markup(mu)
                                    .await?;
                            }
                            None => {
                                let mut mu = InlineKeyboardMarkup::default().append_row(vec![
                                    MyCallback::GetSerialDetails { id: serial.id }.into(),
                                    MyCallback::AddSerialToWatchList { id: serial.id }.into(),
                                ]);
                                if i == l - 1 {
                                    let mut row = Vec::new();
                                    if page != 1 {
                                        row.push(
                                            MyCallback::SearchSerialPreviousPage {
                                                search_string: search_string.clone(),
                                                page: page - 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if total as u8 > page {
                                        row.push(
                                            MyCallback::SearchSerialNextPage {
                                                search_string: search_string.clone(),
                                                page: page + 1,
                                            }
                                            .into(),
                                        );
                                    }
                                    if !row.is_empty() {
                                        mu = mu.append_row(row);
                                    }
                                }
                                bot.send_message(msg.chat.id, text)
                                    .parse_mode(ParseMode::Html)
                                    .reply_markup(mu)
                                    .await?;
                            }
                        }
                    }
                    bot.send_message(msg.chat.id, "Вот результаты поиска")
                        .reply_markup(TextCommand::keyboard())
                        .await?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
