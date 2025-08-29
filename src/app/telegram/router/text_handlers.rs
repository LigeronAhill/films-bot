use anyhow::{Context, Result};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};
use tracing::instrument;

use crate::app::{
    storage::Storage,
    telegram::{MyCallback, MyDialogue, State, TextCommand},
    tmdb::Tmdb,
};

#[instrument(name = "search film by title", skip(bot, msg, dialogue, tmdb_client))]
pub async fn search_film_title_received(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    message_text: String,
    tmdb_client: Tmdb,
) -> Result<()> {
    dialogue.exit().await?;
    let result = tmdb_client.search_film(message_text.clone(), 1).await?;
    let films = result.results;
    let total = result.total_pages;
    if films.is_empty() {
        bot.send_message(msg.chat.id, "Ничего не найдено").await?;
    } else {
        let l = films.len();
        for (i, film) in films.iter().enumerate() {
            let text = film.to_string();
            let file = tmdb_client
                .get_image(&film.poster_path)
                .await
                .context("Getting image")?;
            let mut mu = InlineKeyboardMarkup::default().append_row(vec![
                MyCallback::GetFilmsDetails { id: film.id }.into(),
                MyCallback::AddFilmToWatchList { id: film.id }.into(),
            ]);
            if total > 1 && i == l - 1 {
                mu = mu.append_row(vec![
                    MyCallback::SearchFilmsNextPage {
                        search_string: message_text.clone(),
                        page: 2,
                    }
                    .into(),
                ]);
            }
            bot.send_photo(msg.chat.id, file)
                .caption(&text)
                .parse_mode(ParseMode::Html)
                .reply_markup(mu)
                .await?;
        }
    }

    Ok(())
}
#[instrument(name = "rate film", skip(bot, msg, dialogue, storage))]
pub async fn film_rate_received(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    message_text: String,
    storage: Storage,
) -> Result<()> {
    if let Some(from) = msg.from {
        if let Some(data) = dialogue.get().await? {
            if let State::FilmRateReceived { film_id } = data {
                let user_id = from.id.0;
                let rate = message_text.replace(',', ".").trim().parse()?;
                storage.rate_movie(user_id, film_id, rate).await?;
                bot.send_message(msg.chat.id, "Спасибо за оценку!")
                    .reply_markup(TextCommand::keyboard())
                    .await?;
                dialogue.exit().await?;
            }
        }
    }

    Ok(())
}
