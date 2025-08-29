macro_rules! minutes_to_hours {
    ($minutes:expr) => {{
        let m: i64 = $minutes;
        let hours = m / 60;
        let minutes = m % 60;

        match (hours, minutes) {
            (0, 0) => "0 минут".to_string(),
            (0, m) => format!(
                "{} {}",
                m,
                if m == 1 {
                    "минута"
                } else if (2..=4).contains(&m) {
                    "минуты"
                } else {
                    "минут"
                }
            ),
            (h, 0) => format!(
                "{} {}",
                h,
                if h == 1 {
                    "час"
                } else if (2..=4).contains(&h) {
                    "часа"
                } else {
                    "часов"
                }
            ),
            (h, m) => format!(
                "{} {} {} {}",
                h,
                if h == 1 {
                    "час"
                } else if (2..=4).contains(&h) {
                    "часа"
                } else {
                    "часов"
                },
                m,
                if m == 1 {
                    "минута"
                } else if (2..=4).contains(&m) {
                    "минуты"
                } else {
                    "минут"
                }
            ),
        }
    }};
}
use std::fmt::Display;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use teloxide::types::InputFile;
use tracing::instrument;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Clone)]
pub struct Tmdb {
    token: String,
    client: reqwest::Client,
    base_url: String,
    image_base_url: String,
    language: String,
}
impl Tmdb {
    #[instrument(name = "new tmdb client", skip(token))]
    pub fn new(token: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .gzip(true)
            .user_agent(APP_USER_AGENT)
            .build()?;
        let base_url = String::from("https://api.themoviedb.org/3");
        let image_base_url = String::from("https://image.tmdb.org/t/p/w300");
        let language = String::from("ru");
        Ok(Self {
            token,
            client,
            base_url,
            image_base_url,
            language,
        })
    }
    #[instrument(name = "get image", skip(self))]
    pub async fn get_image(&self, url: &str) -> Result<InputFile> {
        let uri = format!("{b}{url}", b = self.image_base_url);
        tracing::info!("Getting image from {uri}");
        let resp = self
            .client
            .get(&uri)
            .bearer_auth(&self.token)
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("Error: {st}", st = resp.status()));
        }
        let bytes = resp.bytes().await?;
        let f = InputFile::memory(bytes);
        Ok(f)
    }
    #[instrument(name = "search film", skip(self))]
    pub async fn search_film(&self, title: String, page: u8) -> Result<SearchResponse> {
        let uri = format!("{b}/search/movie", b = self.base_url);
        let response = self
            .client
            .get(uri)
            .bearer_auth(&self.token)
            .query(&[
                ("language", self.language.clone()),
                ("query", title),
                ("include_adult", String::from("true")),
                ("page", format!("{page}")),
            ])
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    #[instrument(name = "get films details", skip(self))]
    pub async fn get_films_details(&self, id: i64) -> Result<FilmDetails> {
        let uri = format!("{b}/movie/{id}", b = self.base_url);
        tracing::info!("Getting film details from {u}", u = uri.to_string());
        let result = self
            .client
            .get(uri)
            .bearer_auth(&self.token)
            .query(&[("language", self.language.clone())])
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    #[instrument(name = "get films credits", skip(self))]
    pub async fn get_films_credits(&self, id: i64) -> Result<FilmCredits> {
        let uri = format!("{b}/movie/{id}/credits", b = self.base_url);
        tracing::info!("Getting film credits from {u}", u = uri.to_string());
        let result = self
            .client
            .get(uri)
            .bearer_auth(&self.token)
            .query(&[("language", self.language.clone())])
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    #[instrument(name = "get films credits", skip(self))]
    pub async fn get_popular_movies(&self, page: u8) -> Result<SearchResponse> {
        let uri = format!("{b}/movie/popular", b = self.base_url);
        let response = self
            .client
            .get(uri)
            .bearer_auth(&self.token)
            .query(&[
                ("language", self.language.clone()),
                ("include_adult", String::from("true")),
                ("page", format!("{page}")),
            ])
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub page: i64,
    pub results: Vec<FilmOverview>,
    pub total_pages: i64,
    pub total_results: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilmOverview {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub id: i64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
}
impl Display for FilmOverview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!(
            "Название: <b>{title}</b>\nОригинальное название: <i>{original}</i>\nДата выхода: <i>{release}</i>",
            title = self.title,
            original = self.original_title,
            release = self.release_date
        );
        write!(f, "{text}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilmDetails {
    pub adult: bool,
    pub backdrop_path: String,
    pub belongs_to_collection: Option<BelongsToCollection>,
    pub budget: i64,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: i64,
    pub imdb_id: String,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: i64,
    pub runtime: i64,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
}
impl Display for FilmDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("<b>{title}</b>", title = self.title);
        if let Some(year) = self.release_date.clone().split("-").nth(0) {
            result = format!("{result} ({year})\n");
        } else {
            result.push_str("\n");
        }
        result.push_str(&self.release_date);

        result.push_str(" 🗓️ ");
        let genres = self
            .genres
            .iter()
            .map(|g| g.name.clone())
            .collect::<Vec<_>>();
        for (i, genre) in genres.iter().enumerate() {
            result.push_str(&genre);
            if i != genres.len() - 1 {
                result.push_str(" | ");
            }
        }
        result.push_str(" ⏱️ ");
        let rt = self.runtime;
        let dur = minutes_to_hours!(rt);
        result.push_str(&dur);
        result.push('\n');
        if !self.overview.is_empty() {
            result = format!("{result}<b>Обзор</b>\n{ov}\n", ov = self.overview);
        }
        write!(f, "{result}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductionCompany {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BelongsToCollection {
    pub id: i64,
    pub name: String,
    pub poster_path: String,
    pub backdrop_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilmCredits {
    pub id: i64,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cast {
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub cast_id: i64,
    pub character: String,
    pub credit_id: String,
    pub order: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crew {
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}

use std::fmt;

impl fmt::Display for FilmCredits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "<b>Актерский состав и съемочная группа</b>")?;
        writeln!(f, "👥 <b>Актеров:</b> {}", self.cast.len())?;
        writeln!(f, "🎥 <b>Съемочной группы:</b> {}", self.crew.len())?;
        writeln!(f)?;

        // Главные актеры (первые 10 по порядку)
        if !self.cast.is_empty() {
            writeln!(f, "<b>🎭 Главные роли:</b>")?;
            let main_cast: Vec<&Cast> = self.cast.iter().filter(|c| c.order < 10).take(5).collect();

            for actor in main_cast {
                writeln!(
                    f,
                    "• <b>{}</b> - {}",
                    escape_html(&actor.name),
                    escape_html(&actor.character)
                )?;
            }

            if self.cast.len() > 5 {
                writeln!(f, "<i>... и еще {} актеров</i>", self.cast.len() - 5)?;
            }
            writeln!(f)?;
        }

        // Режиссер
        let director = self
            .crew
            .iter()
            .find(|c| c.job == "Director")
            .map(|d| d.name.clone());

        if let Some(dir) = director {
            writeln!(f, "<b>🎬 Режиссер:</b>")?;
            writeln!(f, "• {}", escape_html(&dir))?;
            writeln!(f)?;
        }

        // Сценаристы
        let writers: Vec<&Crew> = self
            .crew
            .iter()
            .filter(|c| c.job == "Writer" || c.department == "Writing")
            .collect();

        if !writers.is_empty() {
            writeln!(f, "<b>📝 Сценаристы:</b>")?;
            for writer in writers.iter().take(3) {
                writeln!(f, "• {}", escape_html(&writer.name))?;
            }
            writeln!(f)?;
        }

        // Продюсеры
        let producers: Vec<&Crew> = self
            .crew
            .iter()
            .filter(|c| c.job.contains("Producer") || c.department == "Production")
            .collect();

        if !producers.is_empty() {
            writeln!(f, "<b>💰 Продюсеры:</b>")?;
            for producer in producers.iter().take(2) {
                writeln!(f, "• {}", escape_html(&producer.name))?;
            }
            if producers.len() > 2 {
                writeln!(f, "<i>... и еще {} продюсеров</i>", producers.len() - 2)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Cast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<b>{}</b>", escape_html(&self.name))?;

        if !self.character.is_empty() && self.character != "Self" {
            write!(f, " - {}", escape_html(&self.character))?;
        }

        if self.popularity > 10.0 {
            write!(f, " ⭐")?;
        }

        Ok(())
    }
}

impl fmt::Display for Crew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<b>{}</b>", escape_html(&self.name))?;

        if !self.job.is_empty() {
            write!(f, " ({})", escape_html(&self.job))?;
        } else if !self.department.is_empty() {
            write!(f, " ({})", escape_html(&self.department))?;
        }

        Ok(())
    }
}

// Вспомогательная функция для экранирования HTML-символов
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
