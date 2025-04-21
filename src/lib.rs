use crate::models::{
    AnimeList, AnimeStatistics, JikanResponse, MangaList, MangaStatistics, Media, Statistics,
};
use anyhow::{Result, bail};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::env;
use url::Url;

pub mod models;
pub mod svg;
pub mod utils;

const MAL_BASE_URL: &str = "https://api.myanimelist.net/v2/";
const JIKAN_BASE_URL: &str = "https://api.jikan.moe/v4/";

pub struct MalClient {
    client: Client,
    mal_base_url: Url,
    jikan_base_url: Url,
    client_id: String,
}

impl MalClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            mal_base_url: Url::parse(MAL_BASE_URL)?,
            jikan_base_url: Url::parse(JIKAN_BASE_URL)?,
            client_id: env::var("CLIENT_ID")?,
        })
    }

    async fn request_mal<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<T> {
        let mut url = self.mal_base_url.join(path)?;

        if let Some(params) = params {
            let mut pairs = url.query_pairs_mut();

            for (key, value) in params {
                pairs.append_pair(key, value);
            }
        };

        let res = self
            .client
            .get(url)
            .header("X-MAL-CLIENT-ID", &self.client_id)
            .send()
            .await?;

        if !res.status().is_success() {
            bail!(
                "Request to '{}' failed with status code {}: {}",
                res.url().clone(),
                res.status(),
                res.text().await?,
            );
        }

        Ok(res.json::<T>().await?)
    }

    async fn request_jikan<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.jikan_base_url.join(path)?;

        let res = self.client.get(url).send().await?;

        if !res.status().is_success() {
            bail!(
                "Request to '{}' failed with status code {}: {}",
                res.url().clone(),
                res.status(),
                res.text().await?,
            );
        }

        Ok(res.json::<JikanResponse<T>>().await?.data)
    }

    async fn get_user_statistics(&self, user: &str) -> Result<Statistics> {
        self.request_jikan(&format!("users/{}/statistics", user))
            .await
    }

    async fn get_user_activity<T: DeserializeOwned>(
        &self,
        media: Media,
        user: &str,
        limit: Option<u8>,
    ) -> Result<T> {
        let limit = limit.unwrap_or(5).clamp(1, 10).to_string();

        let fields = match media {
            Media::Anime => "list_status,num_episodes",
            Media::Manga => "list_status,num_chapters",
        };

        let params = vec![
            ("fields", fields),
            ("sort", "list_updated_at"),
            ("limit", &limit),
        ];

        self.request_mal(&format!("users/{}/{}list", user, media), Some(params))
            .await
    }

    pub async fn get_user_anime_statistics(&self, user: &str) -> Result<AnimeStatistics> {
        Ok(self.get_user_statistics(user).await?.anime)
    }

    pub async fn get_user_manga_statistics(&self, user: &str) -> Result<MangaStatistics> {
        Ok(self.get_user_statistics(user).await?.manga)
    }

    pub async fn get_user_anime_activity(
        &self,
        user: &str,
        limit: Option<u8>,
    ) -> Result<AnimeList> {
        self.get_user_activity(Media::Anime, user, limit).await
    }

    pub async fn get_user_manga_activity(
        &self,
        user: &str,
        limit: Option<u8>,
    ) -> Result<MangaList> {
        self.get_user_activity(Media::Manga, user, limit).await
    }
}
