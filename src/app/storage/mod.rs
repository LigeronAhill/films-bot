use anyhow::Result;
use futures::TryStreamExt;
use mongodb::{Collection, bson::doc};
use tracing::instrument;

use crate::app::models::{Movie, Serial};

#[derive(Clone, Debug)]
pub struct Storage {
    movies: Collection<Movie>,
    serials: Collection<Serial>,
}

impl Storage {
    #[instrument(name = "new storage", skip_all)]
    pub fn new(movies: Collection<Movie>, serials: Collection<Serial>) -> Self {
        Self { movies, serials }
    }
    #[instrument(name = "get users movies watch list", skip(self))]
    pub async fn get_users_movie_watch_list(&self, user_id: u64) -> Result<Vec<Movie>> {
        let mut cursor = self
            .movies
            .find(doc! {"user_id": user_id as i64, "watched": false})
            .await?;
        let mut result = Vec::new();
        while let Some(movie) = cursor.try_next().await? {
            result.push(movie);
        }
        Ok(result)
    }
    #[instrument(name = "get users watched movies list", skip(self))]
    pub async fn get_users_watched_movies_list(&self, user_id: u64) -> Result<Vec<Movie>> {
        let mut cursor = self
            .movies
            .find(doc! {"user_id": user_id as i64, "watched": true})
            .await?;
        let mut result = Vec::new();
        while let Some(movie) = cursor.try_next().await? {
            result.push(movie);
        }
        Ok(result)
    }
    #[instrument(name = "add film to watch list", skip(self))]
    pub async fn add_film_to_watch_list(&self, user_id: u64, film_id: i64) -> Result<()> {
        let movie = Movie::new(user_id, film_id);
        tracing::info!("Movie: {movie:#?}");
        let current_watch_list = self.get_users_movie_watch_list(user_id).await?;
        if !current_watch_list.is_empty() {
            for movie in current_watch_list {
                if movie.film_id == film_id {
                    return Ok(());
                }
            }
        }
        let res = self.movies.insert_one(movie).await?;
        let c = res.inserted_id.as_str();
        tracing::info!("Inserted id: {c:?}");
        Ok(())
    }
    #[instrument(name = "mark film as watched", skip(self))]
    pub async fn watch_film(&self, user_id: u64, film_id: i64) -> Result<()> {
        let filter = doc! {"user_id": user_id as i64, "film_id": film_id};
        let update = doc! {"$set": doc!{"watched": true}};
        let res = self.movies.update_one(filter, update).await?;
        tracing::info!("Updated {} films in db", res.modified_count);
        Ok(())
    }
    #[instrument(name = "mark film as unwatched", skip(self))]
    pub async fn unwatch_film(&self, user_id: u64, film_id: i64) -> Result<()> {
        let filter = doc! {"user_id": user_id as i64, "film_id": film_id};
        let none_rate: Option<f64> = None;
        let update = doc! {"$set": doc!{"watched": false, "my_rating": none_rate}};
        let res = self.movies.update_one(filter, update).await?;
        tracing::info!("Updated {} films in db", res.modified_count);
        Ok(())
    }
    #[instrument(name = "rate film", skip(self))]
    pub async fn rate_movie(&self, user_id: u64, film_id: i64, rate: f64) -> Result<()> {
        let filter = doc! {"user_id": user_id as i64, "film_id": film_id};
        let update = doc! {"$set": doc!{"my_rating": Some(rate)}};
        let res = self.movies.update_one(filter, update).await?;
        tracing::info!("Updated {} films in db", res.modified_count);
        Ok(())
    }
    #[instrument(name = "delete film from watch list", skip(self))]
    pub async fn delete_film_from_watch_list(&self, user_id: u64, film_id: i64) -> Result<()> {
        let filter = doc! {
            "$and": [
                doc! { "user_id": user_id as i64},
                doc! {"film_id": film_id},
            ]
        };
        let result = self.movies.delete_one(filter).await?;
        tracing::info!("Deleted {} documents", result.deleted_count);
        Ok(())
    }
}
