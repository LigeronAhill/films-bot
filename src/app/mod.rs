pub mod models;
pub mod storage;
pub mod telegram;
pub mod tmdb;

use anyhow::Result;

use models::{Movie, Serial};
use mongodb::{Client, Collection};

const CONTENT_DATABASE: &str = "content";
const MOVIES: &str = "movies";
const SERIALS: &str = "serials";

#[tracing::instrument(name = "app")]
pub async fn run() -> Result<()> {
    tracing_subscriber::fmt().init();
    let mongo_url = std::env::var("MONGODB_URI")?;
    let tmdb_token = std::env::var("TMDB_TOKEN")?;
    let client = Client::with_uri_str(mongo_url).await?;
    let database = client.database(CONTENT_DATABASE);
    let movies_collection: Collection<Movie> = database.collection(MOVIES);
    let serials_collection: Collection<Serial> = database.collection(SERIALS);
    let storage = storage::Storage::new(movies_collection, serials_collection);
    let tmdb_client = tmdb::Tmdb::new(tmdb_token)?;
    telegram::run(storage, tmdb_client).await?;
    Ok(())
}
