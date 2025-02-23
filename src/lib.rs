use crate::models::{AnimeList, MangaList, Media};
use anyhow::{Result, bail};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::env;
use url::Url;

pub mod models;
pub mod svg;
pub mod utils;

const BASE_URL: &str = "https://api.myanimelist.net/v2/";

pub struct MalClient {
    client: Client,
    base_url: Url,
    client_id: String,
}

impl MalClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: Url::parse(BASE_URL)?,
            client_id: env::var("CLIENT_ID")?,
        })
    }

    async fn request<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<T> {
        let mut url = self.base_url.join(path)?;

        if let Some(params) = params {
            let mut pairs = url.query_pairs_mut();

            for (key, value) in params {
                pairs.append_pair(key, &value.to_string());
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

        self.request(
            &format!("users/{}/{}list", user, media.to_string()),
            Some(params),
        )
        .await
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
