use anyhow::{Context, Result};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, KeyboardRemove, ParseMode},
};
use tracing::instrument;

use crate::app::{
    storage::Storage,
    telegram::{MyCallback, MyDialogue, State, TextCommand},
    tmdb::Tmdb,
};

#[instrument(name = "search film", skip_all)]
pub async fn search_film_text_command_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
) -> Result<()> {
    let mu = KeyboardRemove::new();
    bot.send_message(msg.chat.id, "Пришлите название фильма для поиска")
        .reply_markup(mu)
        .await?;
    dialogue.update(State::FilmTitleReceived).await?;
    Ok(())
}
#[instrument(name = "get movies watch list", skip_all)]
pub async fn films_to_watch_text_command_handler(
    bot: Bot,
    msg: Message,
    storage: Storage,
    tmdb_client: Tmdb,
) -> Result<()> {
    tracing::info!("RECEIVED WATCH LIST!");
    if let Some(from) = msg.from {
        let users_watch_list = storage.get_users_movie_watch_list(from.id.0).await?;
        tracing::info!("Users watch list contains {} films", users_watch_list.len());
        if !users_watch_list.is_empty() {
            for movie in users_watch_list {
                let film = tmdb_client.get_films_details(movie.film_id).await?;
                let poster = tmdb_client.get_image(&film.poster_path).await?;
                let mu = InlineKeyboardMarkup::default()
                    .append_row(vec![
                        MyCallback::MarkFilmWatched { id: film.id }.into(),
                        MyCallback::DeleteFilm { id: film.id }.into(),
                    ])
                    .append_row(vec![
                        MyCallback::GetFilmsDetails { id: film.id }.into(),
                        MyCallback::GetFilmsCredits { id: film.id }.into(),
                    ])
                    .append_row(vec![MyCallback::Cancel.into()]);
                bot.send_photo(msg.chat.id, poster)
                    .caption(film.to_string())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(mu)
                    .await?;
            }
        } else {
            let popular_movies = tmdb_client.get_popular_movies(1).await?;
            let watched = storage.get_users_watched_movies_list(from.id.0).await?;
            for film in popular_movies.results {
                if watched.iter().any(|f| f.film_id == film.id) {
                    continue;
                }
                let text = film.to_string();
                match film.poster_path.as_ref() {
                    Some(image_url) => {
                        let file = tmdb_client
                            .get_image(&image_url)
                            .await
                            .context("Getting image")?;
                        let mu = InlineKeyboardMarkup::default().append_row(vec![
                            MyCallback::GetFilmsDetails { id: film.id }.into(),
                            MyCallback::AddFilmToWatchList { id: film.id }.into(),
                        ]);
                        bot.send_photo(msg.chat.id, file)
                            .caption(&text)
                            .parse_mode(ParseMode::Html)
                            .reply_markup(mu)
                            .await?;
                    }
                    None => {
                        let mu = InlineKeyboardMarkup::default().append_row(vec![
                            MyCallback::GetFilmsDetails { id: film.id }.into(),
                            MyCallback::AddFilmToWatchList { id: film.id }.into(),
                        ]);
                        bot.send_message(msg.chat.id, text)
                            .parse_mode(ParseMode::Html)
                            .reply_markup(mu)
                            .await?;
                    }
                }
            }
            bot.send_message(
                msg.chat.id,
                "Ваш список пуст, но вот какие фильмы популярны сейчас ⬆️",
            )
            .await?;
        }
    }
    Ok(())
}
#[instrument(name = "watched movies text command", skip_all)]
pub async fn watched_movies_text_command_handler(
    bot: Bot,
    msg: Message,
    storage: Storage,
    tmdb_client: Tmdb,
) -> Result<()> {
    if let Some(from) = msg.from {
        let watched = storage.get_users_watched_movies_list(from.id.0).await?;
        if !watched.is_empty() {
            for movie in watched {
                let film = tmdb_client.get_films_details(movie.film_id).await?;
                let poster = tmdb_client.get_image(&film.poster_path).await?;
                if let Some(current_rate) = movie.my_rating {
                    let mu = InlineKeyboardMarkup::default()
                        .append_row(vec![
                            MyCallback::RateFilm { id: film.id }.into(),
                            MyCallback::MarkFilmUnWatched { id: film.id }.into(),
                        ])
                        .append_row(vec![MyCallback::DeleteFilm { id: film.id }.into()]);
                    bot.send_photo(msg.chat.id, poster)
                        .caption(format!(
                            "{}\nВаша текущая оценка: {current_rate:.2}",
                            film.to_string()
                        ))
                        .parse_mode(ParseMode::Html)
                        .reply_markup(mu)
                        .await?;
                } else {
                    let mu = InlineKeyboardMarkup::default().append_row(vec![
                        MyCallback::RateFilm { id: film.id }.into(),
                        MyCallback::DeleteFilm { id: film.id }.into(),
                    ]);
                    bot.send_photo(msg.chat.id, poster)
                        .caption(film.to_string())
                        .parse_mode(ParseMode::Html)
                        .reply_markup(mu)
                        .await?;
                }
            }
        } else {
            bot.send_message(msg.chat.id, "Ваш список просмотренных фильмов пуст")
                .reply_markup(TextCommand::keyboard())
                .await?;
        }
    }
    Ok(())
}
#[instrument(name = "search serial", skip_all)]
pub async fn search_serial_text_command_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
) -> Result<()> {
    let mu = KeyboardRemove::new();
    bot.send_message(msg.chat.id, "Пришлите название сериала для поиска")
        .reply_markup(mu)
        .await?;
    dialogue.update(State::SerialTitleReceived).await?;
    Ok(())
}
#[instrument(name = "get serials watch list", skip_all)]
pub async fn serials_to_watch_text_command_handler(
    bot: Bot,
    msg: Message,
    storage: Storage,
    tmdb_client: Tmdb,
) -> Result<()> {
    if let Some(from) = msg.from {
        let users_watch_list = storage.get_users_serials_watch_list(from.id.0).await?;
        tracing::info!(
            "Users watch list contains {} serials",
            users_watch_list.len()
        );
        if !users_watch_list.is_empty() {
            for serial in users_watch_list {
                let tv_show = tmdb_client.get_tv_show_details(serial.serial_id).await?;
                let poster = tmdb_client.get_image(&tv_show.poster_path).await?;
                let mu = InlineKeyboardMarkup::default()
                    .append_row(vec![
                        MyCallback::MarkSerialWatched { id: tv_show.id }.into(),
                        MyCallback::DeleteSerial { id: tv_show.id }.into(),
                    ])
                    .append_row(vec![
                        MyCallback::GetSerialDetails { id: tv_show.id }.into(),
                        MyCallback::GetSerialCredits { id: tv_show.id }.into(),
                    ])
                    .append_row(vec![MyCallback::Cancel.into()]);
                bot.send_photo(msg.chat.id, poster)
                    .caption(tv_show.to_string())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(mu)
                    .await?;
            }
        } else {
            let popular_serials = tmdb_client.get_popular_tv_shows(1).await?;
            let watched = storage.get_users_watched_serials_list(from.id.0).await?;
            for serial in popular_serials.results {
                if watched.iter().any(|f| f.serial_id == serial.id) {
                    continue;
                }
                let text = serial.to_string();
                match serial.poster_path.as_ref() {
                    Some(image_url) => {
                        let file = tmdb_client
                            .get_image(&image_url)
                            .await
                            .context("Getting image")?;
                        let mu = InlineKeyboardMarkup::default().append_row(vec![
                            MyCallback::GetSerialDetails { id: serial.id }.into(),
                            MyCallback::AddSerialToWatchList { id: serial.id }.into(),
                        ]);
                        bot.send_photo(msg.chat.id, file)
                            .caption(&text)
                            .parse_mode(ParseMode::Html)
                            .reply_markup(mu)
                            .await?;
                    }
                    None => {
                        let mu = InlineKeyboardMarkup::default().append_row(vec![
                            MyCallback::GetSerialDetails { id: serial.id }.into(),
                            MyCallback::AddSerialToWatchList { id: serial.id }.into(),
                        ]);
                        bot.send_message(msg.chat.id, text)
                            .parse_mode(ParseMode::Html)
                            .reply_markup(mu)
                            .await?;
                    }
                }
            }
            bot.send_message(
                msg.chat.id,
                "Ваш список пуст, но вот какие фильмы популярны сейчас ⬆️",
            )
            .await?;
        }
    }
    Ok(())
}
#[instrument(name = "watched serials text command", skip_all)]
pub async fn watched_serials_text_command_handler(
    bot: Bot,
    msg: Message,
    storage: Storage,
    tmdb_client: Tmdb,
) -> Result<()> {
    if let Some(from) = msg.from {
        let watched = storage.get_users_watched_serials_list(from.id.0).await?;
        if !watched.is_empty() {
            for serial in watched {
                let tv_show = tmdb_client.get_tv_show_details(serial.serial_id).await?;
                let poster = tmdb_client.get_image(&tv_show.poster_path).await?;
                if let Some(current_rate) = serial.my_rating {
                    let mu = InlineKeyboardMarkup::default()
                        .append_row(vec![
                            MyCallback::RateSerial { id: tv_show.id }.into(),
                            MyCallback::MarkSerialUnWatched { id: tv_show.id }.into(),
                        ])
                        .append_row(vec![MyCallback::DeleteSerial { id: tv_show.id }.into()]);
                    bot.send_photo(msg.chat.id, poster)
                        .caption(format!(
                            "{}\nВаша текущая оценка: {current_rate:.2}",
                            tv_show.to_string()
                        ))
                        .parse_mode(ParseMode::Html)
                        .reply_markup(mu)
                        .await?;
                } else {
                    let mu = InlineKeyboardMarkup::default().append_row(vec![
                        MyCallback::RateSerial { id: tv_show.id }.into(),
                        MyCallback::DeleteSerial { id: tv_show.id }.into(),
                    ]);
                    bot.send_photo(msg.chat.id, poster)
                        .caption(tv_show.to_string())
                        .parse_mode(ParseMode::Html)
                        .reply_markup(mu)
                        .await?;
                }
            }
        } else {
            bot.send_message(msg.chat.id, "Ваш список просмотренных фильмов пуст")
                .reply_markup(TextCommand::keyboard())
                .await?;
        }
    }
    Ok(())
}
