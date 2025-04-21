use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Media {
    Anime,
    Manga,
}

#[derive(Deserialize)]
pub struct JikanResponse<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct Statistics {
    pub anime: AnimeStatistics,
    pub manga: MangaStatistics,
}

#[derive(Deserialize)]
pub struct AnimeStatistics {
    pub days_watched: f32,
    pub mean_score: f32,
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
    pub total_entries: u32,
    pub rewatched: u32,
    pub episodes_watched: u32,
}

#[derive(Deserialize)]
pub struct MangaStatistics {
    pub days_read: f32,
    pub mean_score: f32,
    pub reading: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_read: u32,
    pub total_entries: u32,
    pub reread: u32,
    pub chapters_read: u32,
    pub volumes_read: u32,
}

#[derive(Deserialize)]
pub struct AnimeList {
    pub data: Vec<AnimeEntry>,
}

#[derive(Deserialize)]
pub struct AnimeEntry {
    pub node: AnimeNode,
    pub list_status: AnimeListStatus,
}

#[derive(Deserialize)]
pub struct AnimeNode {
    pub id: u32,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub num_episodes: u32,
}

#[derive(Deserialize)]
pub struct AnimeListStatus {
    pub status: WatchStatus,
    pub score: u8,
    pub num_episodes_watched: u32,
    pub is_rewatching: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Display, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WatchStatus {
    Watching,
    Completed,
    #[strum(serialize = "On Hold")]
    OnHold,
    Dropped,
    #[strum(serialize = "Plan to Watch")]
    PlanToWatch,
}

#[derive(Deserialize)]
pub struct MangaList {
    pub data: Vec<MangaEntry>,
}

#[derive(Deserialize)]
pub struct MangaEntry {
    pub node: MangaNode,
    pub list_status: MangaListStatus,
}

#[derive(Deserialize)]
pub struct MangaNode {
    pub id: u32,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub num_chapters: u32,
}

#[derive(Deserialize)]
pub struct MangaListStatus {
    pub status: ReadStatus,
    pub is_rereading: bool,
    pub num_volumes_read: u32,
    pub num_chapters_read: u32,
    pub score: u8,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Display, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReadStatus {
    Reading,
    Completed,
    #[strum(serialize = "On Hold")]
    OnHold,
    Dropped,
    #[strum(serialize = "Plan to Read")]
    PlanToRead,
}

#[derive(Deserialize)]
pub struct Picture {
    pub medium: String,
    pub large: Option<String>,
}
