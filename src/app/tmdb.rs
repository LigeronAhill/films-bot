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
    #[instrument(name = "get popular movies", skip(self))]
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
    #[instrument(name = "search tv show", skip(self))]
    pub async fn search_tvshow(&self, title: String, page: u8) -> Result<SearchTVResponse> {
        let uri = format!("{b}/search/tv", b = self.base_url);
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
    #[instrument(name = "get tv show details", skip(self))]
    pub async fn get_tv_show_details(&self, id: i64) -> Result<TVShowDetails> {
        let uri = format!("{b}/tv/{id}", b = self.base_url);
        tracing::info!("Getting tv show details from {u}", u = uri.to_string());
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
    #[instrument(name = "get tv show credits", skip(self))]
    pub async fn get_tv_show_credits(&self, id: i64) -> Result<FilmCredits> {
        let uri = format!("{b}/tv/{id}/credits", b = self.base_url);
        tracing::info!("Getting tv show credits from {u}", u = uri.to_string());
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
    #[instrument(name = "get popular tv shows", skip(self))]
    pub async fn get_popular_tv_shows(&self, page: u8) -> Result<SearchTVResponse> {
        let uri = format!("{b}/tv/popular", b = self.base_url);
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
    pub poster_path: Option<String>,
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
            if self.overview.chars().count() < 512 {
                result = format!(
                    "{result}<b>Обзор</b>\n{overview}\n",
                    overview = self.overview
                );
            } else {
                let short: String = self.overview.clone().chars().take(512).collect();
                result = format!("{result}<b>Обзор</b>\n{short}...\n");
            }
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
    pub origin_country: Option<String>,
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
    pub cast_id: Option<i64>,
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTVResponse {
    pub page: i64,
    pub results: Vec<TVShowOverview>,
    pub total_pages: i64,
    pub total_results: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TVShowOverview {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub id: i64,
    pub original_language: String,
    pub original_country: Option<Vec<String>>,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub first_air_date: String,
    pub name: String,
    pub vote_average: f64,
    pub vote_count: i64,
}
impl Display for TVShowOverview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = escape_html(&self.name);
        let original_name = escape_html(&self.original_name);
        let overview = escape_html(&self.overview);

        // Форматируем дату (если есть)
        let air_date = if self.first_air_date.is_empty() {
            "Неизвестно".to_string()
        } else {
            self.first_air_date.clone()
        };

        // Форматируем рейтинг
        let rating = format!("{:.1}", self.vote_average);
        let votes = self.vote_count;

        // Строим HTML-разметку
        write!(f, "<b>🎬 {}</b>", name)?;

        if self.original_name != self.name {
            write!(f, "\n<code>({})</code>", original_name)?;
        }

        write!(f, "\n\n📅 <b>Премьера:</b> {}", air_date)?;
        write!(f, "\n⭐ <b>Рейтинг:</b> {} ({} голосов)", rating, votes)?;
        write!(f, "\n🌐 <b>Язык:</b> {}", self.original_language)?;

        if let Some(original_country) = self.original_country.as_ref() {
            if !original_country.is_empty() {
                write!(f, "\n🇺🇳 <b>Страна:</b> {}", original_country.join(", "))?;
            }
        }

        if !overview.is_empty() {
            if overview.chars().count() > 512 {
                let short: String = overview.chars().take(512).collect();
                write!(f, "\n\n📖 <b>Описание:</b>\n{short}...")?;
            } else {
                write!(f, "\n\n📖 <b>Описание:</b>\n{overview}")?;
            }
        }

        // Добавляем информацию о возрастном ограничении, если есть
        if self.adult {
            write!(f, "\n\n🔞 <b>18+</b>")?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TVShowDetails {
    pub adult: bool,
    pub backdrop_path: String,
    pub created_by: Vec<CreatedBy>,
    pub episode_run_time: Vec<i64>,
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: i64,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub last_episode_to_air: LastEpisodeToAir,
    pub name: String,
    pub next_episode_to_air: Option<NextEpisodeToAir>,
    pub networks: Vec<Network>,
    pub number_of_episodes: i64,
    pub number_of_seasons: i64,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub seasons: Vec<Season>,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub vote_average: f64,
    pub vote_count: i64,
}
impl Display for TVShowDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = escape_html(&self.name);
        let original_name = escape_html(&self.original_name);
        let overview = escape_html(&self.overview);
        let tagline = escape_html(&self.tagline);
        let status = escape_html(&self.status);

        // Основная информация
        write!(f, "<b>🎬 {}</b>", name)?;

        if self.original_name != self.name {
            write!(f, "\n<code>({})</code>", original_name)?;
        }

        if !tagline.is_empty() {
            write!(f, "\n<em>{}</em>", tagline)?;
        }

        // Даты выхода
        write!(f, "\n\n📅 <b>Премьера:</b> {}", self.first_air_date)?;
        if !self.last_air_date.is_empty() {
            write!(f, "\n📅 <b>Последний эфир:</b> {}", self.last_air_date)?;
        }

        // Рейтинг и статистика
        write!(
            f,
            "\n⭐ <b>Рейтинг:</b> {:.1} ({} голосов)",
            self.vote_average, self.vote_count
        )?;
        write!(f, "\n📊 <b>Сезонов:</b> {}", self.number_of_seasons)?;
        write!(f, "\n🎞️ <b>Эпизодов:</b> {}", self.number_of_episodes)?;

        // Жанры
        if !self.genres.is_empty() {
            let genres: Vec<String> = self.genres.iter().map(|g| escape_html(&g.name)).collect();
            write!(f, "\n🎭 <b>Жанры:</b> {}", genres.join(", "))?;
        }

        // Статус производства
        write!(f, "\n🔄 <b>Статус:</b> {}", status)?;
        if self.in_production {
            write!(f, " 🎬")?;
        }

        // Страны производства
        if !self.origin_country.is_empty() {
            write!(f, "\n🇺🇳 <b>Страны:</b> {}", self.origin_country.join(", "))?;
        }

        // Языки
        if !self.languages.is_empty() {
            write!(f, "\n🌐 <b>Языки:</b> {}", self.languages.join(", "))?;
        }

        // Создатели
        if !self.created_by.is_empty() {
            let creators: Vec<String> = self
                .created_by
                .iter()
                .map(|c| escape_html(&c.name))
                .collect();
            write!(f, "\n👨‍💼 <b>Создатели:</b> {}", creators.join(", "))?;
        }

        // Сети вещания
        if !self.networks.is_empty() {
            let networks: Vec<String> =
                self.networks.iter().map(|n| escape_html(&n.name)).collect();
            write!(f, "\n📺 <b>Телеканалы:</b> {}", networks.join(", "))?;
        }

        // Время эпизодов
        if !self.episode_run_time.is_empty() {
            let run_times: Vec<String> = self
                .episode_run_time
                .iter()
                .map(|t| format!("{} мин", t))
                .collect();
            write!(f, "\n⏱️ <b>Длительность:</b> {}", run_times.join(", "))?;
        }

        // Последний эпизод
        write!(f, "\n\n📺 <b>Последний эпизод:</b>")?;
        write!(
            f,
            "\n   • <b>Название:</b> {}",
            escape_html(&self.last_episode_to_air.name)
        )?;
        write!(
            f,
            "\n   • <b>Дата:</b> {}",
            self.last_episode_to_air.air_date
        )?;
        write!(
            f,
            "\n   • <b>Сезон:</b> {}",
            self.last_episode_to_air.season_number
        )?;
        write!(
            f,
            "\n   • <b>Эпизод:</b> {}",
            self.last_episode_to_air.episode_number
        )?;

        // Следующий эпизод (если есть)
        if let Some(next_episode) = &self.next_episode_to_air {
            write!(f, "\n\n📺 <b>Следующий эпизод:</b>")?;
            write!(
                f,
                "\n   • <b>Название:</b> {}",
                escape_html(&next_episode.name)
            )?;
            write!(f, "\n   • <b>Дата:</b> {}", next_episode.air_date)?;
            write!(f, "\n   • <b>Сезон:</b> {}", next_episode.season_number)?;
            write!(f, "\n   • <b>Эпизод:</b> {}", next_episode.episode_number)?;
        }

        // Описание
        if !overview.is_empty() {
            if overview.chars().count() > 512 {
                let short: String = overview.chars().take(512).collect();
                write!(f, "\n\n📖 <b>Описание:</b>\n{short}...")?;
            } else {
                write!(f, "\n\n📖 <b>Описание:</b>\n{overview}")?;
            }
        }

        // Возрастное ограничение
        if self.adult {
            write!(f, "\n\n🔞 <b>18+</b>")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatedBy {
    pub id: i64,
    pub credit_id: String,
    pub name: String,
    pub original_name: String,
    pub gender: i64,
    pub profile_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LastEpisodeToAir {
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub vote_average: f64,
    pub vote_count: i64,
    pub air_date: String,
    pub episode_number: i64,
    pub episode_type: String,
    pub production_code: String,
    pub runtime: i64,
    pub season_number: i64,
    pub show_id: i64,
    pub still_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NextEpisodeToAir {
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub vote_average: f64,
    pub vote_count: i64,
    pub air_date: String,
    pub episode_number: i64,
    pub episode_type: String,
    pub production_code: String,
    pub runtime: Option<i64>,
    pub season_number: i64,
    pub show_id: i64,
    pub still_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: Option<String>,
}
impl Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name,
            self.origin_country.clone().unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Season {
    pub air_date: String,
    pub episode_count: i64,
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub season_number: i64,
    pub vote_average: f64,
}
impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "📀 <b>{}:</b>", escape_html(&self.name))?;
        write!(f, "\n   • <b>Эпизодов:</b> {}", self.episode_count)?;
        write!(f, "\n   • <b>Дата выхода:</b> {}", self.air_date)?;
        if self.vote_average > 0.0 {
            write!(f, "\n   • <b>Рейтинг:</b> {:.1}", self.vote_average)?;
        }

        Ok(())
    }
}
