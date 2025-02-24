use mal_readme_stats::MalClient;
use mal_readme_stats::models::Media;
use mal_readme_stats::svg::ToSvg;
use mal_readme_stats::utils::QueryParams;
use std::str::FromStr;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let params = QueryParams::from_request(&req)?;

    let media = Media::from_str(params.get("media").unwrap())?;
    let user = params.get("user").unwrap();
    let limit = params.get("limit").and_then(|s| s.parse().ok());

    let client = MalClient::new()?;

    let svg = match media {
        Media::Anime => {
            client
                .get_user_anime_activity(user, limit)
                .await?
                .to_svg()
                .await
        }
        Media::Manga => {
            client
                .get_user_manga_activity(user, limit)
                .await?
                .to_svg()
                .await
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .body(svg.into())?)
}
