mod router;
use std::{fmt::Display, str::FromStr};

use anyhow::{Result, anyhow};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{InlineKeyboardButton, KeyboardButton, KeyboardMarkup},
    utils::command::BotCommands,
};

use crate::app::{storage::Storage, tmdb::Tmdb};

const CANCEL_CALLBACK: &str = "cancel";
const SEARCH_FILM_CALLBACK: &str = "search_films";
const GET_FILM_DETAILS_CALLBACK: &str = "get_films_details";
const GET_FILM_CREDITS_CALLBACK: &str = "get_films_credits";
const ADD_FILM_TO_WATCH_LIST_CALLBACK: &str = "add_film_to_watch_list";
const MARK_FILM_WATCHED_CALLBACK: &str = "mark_film_watched";
const MARK_FILM_UNWATCHED_CALLBACK: &str = "mark_film_unwatched";
const RATE_FILM_CALLBACK: &str = "rate_film";
const DELETE_FILM_CALLBACK: &str = "delete_film";

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    FilmTitleReceived,
    FilmRateReceived {
        film_id: i64,
    },
}

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

/// These commands are supported:
#[derive(teloxide::macros::BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    /// Display this text.
    Help,
    /// Get main menu.
    Start,
    /// Cancel.
    Cancel,
}

const TO_WATCH: &str = "🤔 Что посмотреть?";
const SEARCH_FILM: &str = "🔎 Найти фильм";
const WATCHED_FILMS: &str = "💼 Просмотренные фильмы";
const SEARCH_SERIAL: &str = "🔎 Найти сериал";
const WATCHED_SERIALS: &str = "💼 Просмотренные сериалы";

#[derive(Debug, Clone)]
pub enum TextCommand {
    ToWatch,
    SearchFilm,
    WatchedFilms,
    SearchSerial,
    WatchedSerials,
}
impl TextCommand {
    pub fn keyboard() -> KeyboardMarkup {
        KeyboardMarkup::default()
            .append_row(vec![TextCommand::ToWatch.into()])
            .append_row(vec![
                TextCommand::SearchFilm.into(),
                TextCommand::SearchSerial.into(),
            ])
            .append_row(vec![
                TextCommand::WatchedFilms.into(),
                TextCommand::WatchedSerials.into(),
            ])
            .resize_keyboard()
    }
}
impl From<TextCommand> for KeyboardButton {
    fn from(value: TextCommand) -> Self {
        KeyboardButton::new(value.to_string())
    }
}
impl Display for TextCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            TextCommand::ToWatch => TO_WATCH,
            TextCommand::SearchFilm => SEARCH_FILM,
            TextCommand::WatchedFilms => WATCHED_FILMS,
            TextCommand::SearchSerial => SEARCH_SERIAL,
            TextCommand::WatchedSerials => WATCHED_SERIALS,
        };
        write!(f, "{string}")
    }
}
impl FromStr for TextCommand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            TO_WATCH => Ok(Self::ToWatch),
            SEARCH_FILM => Ok(Self::SearchFilm),
            WATCHED_FILMS => Ok(Self::WatchedFilms),
            SEARCH_SERIAL => Ok(Self::SearchSerial),
            WATCHED_SERIALS => Ok(Self::WatchedSerials),
            _ => Err(anyhow!("Not a text command")),
        }
    }
}
#[derive(Clone, Debug)]
pub enum MyCallback {
    Cancel,
    SearchFilmsNextPage { search_string: String, page: u8 },
    SearchFilmsPreviousPage { search_string: String, page: u8 },
    GetFilmsDetails { id: i64 },
    GetFilmsCredits { id: i64 },
    AddFilmToWatchList { id: i64 },
    MarkFilmWatched { id: i64 },
    MarkFilmUnWatched { id: i64 },
    RateFilm { id: i64 },
    DeleteFilm { id: i64 },
}
impl MyCallback {
    fn data(&self) -> String {
        match self {
            MyCallback::Cancel => CANCEL_CALLBACK.into(),
            MyCallback::SearchFilmsNextPage {
                search_string,
                page,
            }
            | MyCallback::SearchFilmsPreviousPage {
                search_string,
                page,
            } => format!("{SEARCH_FILM_CALLBACK}:{search_string}:{page}"),
            MyCallback::GetFilmsDetails { id } => format!("{GET_FILM_DETAILS_CALLBACK}:{id}"),
            MyCallback::AddFilmToWatchList { id } => {
                format!("{ADD_FILM_TO_WATCH_LIST_CALLBACK}:{id}")
            }
            MyCallback::GetFilmsCredits { id } => format!("{GET_FILM_CREDITS_CALLBACK}:{id}"),
            MyCallback::MarkFilmWatched { id } => format!("{MARK_FILM_WATCHED_CALLBACK}:{id}"),
            MyCallback::RateFilm { id } => format!("{RATE_FILM_CALLBACK}:{id}"),
            MyCallback::MarkFilmUnWatched { id } => format!("{MARK_FILM_UNWATCHED_CALLBACK}:{id}"),
            MyCallback::DeleteFilm { id } => format!("{DELETE_FILM_CALLBACK}:{id}"),
        }
    }
}
impl From<MyCallback> for InlineKeyboardButton {
    fn from(value: MyCallback) -> Self {
        InlineKeyboardButton::callback(value.to_string(), value.data())
    }
}
impl Display for MyCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            MyCallback::Cancel => "🔙 Вернуться в меню",
            MyCallback::SearchFilmsNextPage { .. } => "⏭️ Дальше",
            MyCallback::SearchFilmsPreviousPage { .. } => "⏮️ Назад",
            MyCallback::GetFilmsDetails { .. } => "🕵️ Подробнее",
            MyCallback::AddFilmToWatchList { .. } => "🤔 Буду смотреть",
            MyCallback::GetFilmsCredits { .. } => "⚙️ Титры",
            MyCallback::MarkFilmWatched { .. } => "✅ Отметить просмотренным",
            MyCallback::RateFilm { .. } => "🧮 Поставить оценку",
            MyCallback::MarkFilmUnWatched { .. } => "👁️  Отметить непросмотренным",
            MyCallback::DeleteFilm { .. } => "🗑️ Удалить из списка",
        };
        write!(f, "{string}")
    }
}
impl FromStr for MyCallback {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == CANCEL_CALLBACK {
            return Ok(Self::Cancel);
        } else if let Some((action, data)) = s.split_once(':') {
            match action {
                SEARCH_FILM_CALLBACK => {
                    if let Some((search_string, page)) = data.split_once(':') {
                        let page = page.parse()?;
                        let search_string = search_string.into();
                        return Ok(Self::SearchFilmsNextPage {
                            search_string,
                            page,
                        });
                    }
                }
                GET_FILM_DETAILS_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::GetFilmsDetails { id });
                }
                ADD_FILM_TO_WATCH_LIST_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::AddFilmToWatchList { id });
                }
                GET_FILM_CREDITS_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::GetFilmsCredits { id });
                }
                MARK_FILM_WATCHED_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::MarkFilmWatched { id });
                }
                MARK_FILM_UNWATCHED_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::MarkFilmUnWatched { id });
                }
                RATE_FILM_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::RateFilm { id });
                }
                DELETE_FILM_CALLBACK => {
                    let id = data.parse()?;
                    return Ok(Self::DeleteFilm { id });
                }
                _ => {}
            }
        }
        Err(anyhow!("Not a callback"))
    }
}
#[tracing::instrument(name = "telegram bot", skip_all)]
pub async fn run(storage: Storage, tmdb_client: Tmdb) -> Result<()> {
    let bot = Bot::from_env();
    tracing::info!("🚀 Starting 🤖  bot");
    bot.delete_webhook().drop_pending_updates(true).await?;
    bot.set_my_commands(Command::bot_commands()).await?;
    Dispatcher::builder(bot, router::main_router())
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            storage,
            tmdb_client
        ])
        .default_handler(|upd| async move {
            tracing::warn!("Unhandled update: {upd:?}");
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
