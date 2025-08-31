mod command_handlers;
use command_handlers::*;
mod callback_handlers;
use callback_handlers::*;
mod text_command_handlers;
use text_command_handlers::*;
mod text_handlers;
use text_handlers::*;

use anyhow::Error;
use std::str::FromStr;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{UpdateHandler, dialogue};
use teloxide::prelude::*;

use crate::app::telegram::{Command, MyCallback, State, TextCommand};

pub fn main_router() -> UpdateHandler<Error> {
    use dptree::case;
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(start_command_handler))
        .branch(case![Command::Help].endpoint(help_command_handler))
        .branch(case![Command::Cancel].endpoint(cancel_command_handler));
    let callback_handler = Update::filter_callback_query()
        .filter_map(my_callback_projection)
        .branch(case![MyCallback::Cancel].endpoint(cancel_callback_handler))
        .branch(
            case![MyCallback::GetFilmsDetails { id }].endpoint(get_film_details_callback_handler),
        )
        .branch(
            case![MyCallback::GetSerialDetails { id }]
                .endpoint(get_serial_details_callback_handler),
        )
        .branch(
            case![MyCallback::GetFilmsCredits { id }].endpoint(get_film_credits_callback_handler),
        )
        .branch(
            case![MyCallback::GetSerialCredits { id }]
                .endpoint(get_serial_credits_callback_handler),
        )
        .branch(
            case![MyCallback::AddFilmToWatchList { id }]
                .endpoint(add_film_to_watch_list_callback_handler),
        )
        .branch(
            case![MyCallback::AddSerialToWatchList { id }]
                .endpoint(add_serial_to_watch_list_callback_handler),
        )
        .branch(
            case![MyCallback::MarkFilmWatched { id }].endpoint(mark_film_watched_callback_handler),
        )
        .branch(
            case![MyCallback::MarkSerialWatched { id }]
                .endpoint(mark_serial_watched_callback_handler),
        )
        .branch(
            case![MyCallback::MarkFilmUnWatched { id }]
                .endpoint(mark_film_unwatched_callback_handler),
        )
        .branch(
            case![MyCallback::MarkSerialUnWatched { id }]
                .endpoint(mark_serial_unwatched_callback_handler),
        )
        .branch(case![MyCallback::RateFilm { id }].endpoint(rate_film_callback_handler))
        .branch(case![MyCallback::RateSerial { id }].endpoint(rate_serial_callback_handler))
        .branch(
            case![MyCallback::DeleteFilm { id }].endpoint(delete_film_from_list_callback_handler),
        )
        .branch(
            case![MyCallback::DeleteSerial { id }]
                .endpoint(delete_serial_from_list_callback_handler),
        )
        .branch(
            case![MyCallback::SearchFilmsNextPage {
                search_string,
                page
            }]
            .endpoint(search_film_pagination_callback_handler),
        )
        .branch(
            case![MyCallback::SearchFilmsPreviousPage {
                search_string,
                page
            }]
            .endpoint(search_film_pagination_callback_handler),
        )
        .branch(
            case![MyCallback::SearchSerialNextPage {
                search_string,
                page
            }]
            .endpoint(search_serial_pagination_callback_handler),
        )
        .branch(
            case![MyCallback::SearchSerialPreviousPage {
                search_string,
                page
            }]
            .endpoint(search_serial_pagination_callback_handler),
        );
    let text_command_handler = Update::filter_message()
        .filter_map(text_command_projection)
        .branch(case![TextCommand::FilmsToWatch].endpoint(films_to_watch_text_command_handler))
        .branch(case![TextCommand::SerialsToWatch].endpoint(serials_to_watch_text_command_handler))
        .branch(case![TextCommand::SearchFilm].endpoint(search_film_text_command_handler))
        .branch(case![TextCommand::SearchSerial].endpoint(search_serial_text_command_handler))
        .branch(case![TextCommand::WatchedFilms].endpoint(watched_movies_text_command_handler))
        .branch(case![TextCommand::WatchedSerials].endpoint(watched_serials_text_command_handler));
    let state_handler = Update::filter_message().branch(
        Message::filter_text()
            .branch(case![State::FilmTitleReceived].endpoint(search_film_title_received))
            .branch(case![State::SerialTitleReceived].endpoint(search_serial_title_received))
            .branch(case![State::FilmRateReceived { film_id }].endpoint(film_rate_received))
            .branch(case![State::SerialRateReceived { serial_id }].endpoint(serial_rate_received)),
    );
    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(text_command_handler)
        .branch(state_handler);

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_handler)
}
fn text_command_projection(msg: Message) -> Option<TextCommand> {
    let command = msg.text()?;
    if let Ok(tc) = TextCommand::from_str(command) {
        return Some(tc);
    }
    None
}

fn my_callback_projection(q: CallbackQuery) -> Option<MyCallback> {
    let s = q.data?;
    MyCallback::from_str(&s).ok()
}
