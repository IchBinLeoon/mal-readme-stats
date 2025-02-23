use crate::models::{AnimeList, MangaList, ReadStatus, WatchStatus};
use crate::utils::fetch_image_base64;
use quick_xml::escape::minimal_escape;

const ACTIVITY_PADDING: usize = 15;
const ACTIVITY_WIDTH: usize = 500 + 2 * ACTIVITY_PADDING;
const ACTIVITY_ENTRY_HEIGHT: usize = 100;

const ACTIVITY_IMAGE_WIDTH: usize = 70;
const ACTIVITY_IMAGE_HEIGHT: usize = 100;

const ACTIVITY_X_TEXT_OFFSET: usize = 2 * ACTIVITY_PADDING + ACTIVITY_IMAGE_WIDTH;
const ACTIVITY_TITLE_MAX_LENGTH: usize = 35;

const ACTIVITY_BAR_PADDING: usize = 2;
const ACTIVITY_BAR_WIDTH: usize = ACTIVITY_WIDTH - ACTIVITY_X_TEXT_OFFSET - ACTIVITY_PADDING;
const ACTIVITY_BAR_HEIGHT: usize = 16;

const COLOR_GREEN: &str = "#00cc66";
const COLOR_BLUE: &str = "#3399cc";
const COLOR_YELLOW: &str = "#ffcc00";
const COLOR_RED: &str = "#cc3333";
const COLOR_GRAY: &str = "#6f6f6f";
const COLOR_BACKGROUND: &str = "#111111";

pub trait ToSvg {
    fn to_svg(&self) -> impl Future<Output = String>;
}

impl ToSvg for AnimeList {
    async fn to_svg(&self) -> String {
        let mut svg = String::new();

        let height =
            self.data.len() * (ACTIVITY_ENTRY_HEIGHT + ACTIVITY_PADDING) + ACTIVITY_PADDING;

        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
            ACTIVITY_WIDTH, height
        ));

        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="{}" rx="10"/>"#,
            ACTIVITY_WIDTH, height, COLOR_BACKGROUND
        ));

        for (index, entry) in self.data.iter().enumerate() {
            let y_offset = index * ACTIVITY_ENTRY_HEIGHT + (index + 1) * ACTIVITY_PADDING;

            if let Some(picture) = &entry.node.main_picture {
                if let Some(image) = fetch_image_base64(&picture.medium).await {
                    svg.push_str(&format!(
                        r#"<a href="https://myanimelist.net/anime/{}" target="_blank"><image x="{}" y="{}" width="{}" height="{}" href="data:image/png;base64,{}"/></a>"#,
                        entry.node.id, ACTIVITY_PADDING, y_offset, ACTIVITY_IMAGE_WIDTH, ACTIVITY_IMAGE_HEIGHT, image
                    ));
                }
            }

            let title = if entry.node.title.len() > ACTIVITY_TITLE_MAX_LENGTH {
                format!("{}...", &entry.node.title[..ACTIVITY_TITLE_MAX_LENGTH])
            } else {
                entry.node.title.clone()
            };

            svg.push_str(&format!(
                r#"<a href="https://myanimelist.net/anime/{}" target="_blank"><text x="{}" y="{}" font-family="Arial" font-size="20" fill="white">{}</text></a>"#,
                entry.node.id,
                ACTIVITY_X_TEXT_OFFSET,
                y_offset + 20,
                minimal_escape(title)
            ));

            if entry.list_status.score > 0 {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-family="Arial" font-size="16" fill="white" text-anchor="end">★ {}</text>"#,
                    ACTIVITY_WIDTH - ACTIVITY_PADDING,
                    y_offset + 20,
                    entry.list_status.score,
                ));
            }

            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="3"/>"#,
                ACTIVITY_X_TEXT_OFFSET,
                y_offset + 40,
                ACTIVITY_BAR_WIDTH,
                ACTIVITY_BAR_HEIGHT,
                COLOR_GRAY
            ));

            let color = match entry.list_status.status {
                WatchStatus::Watching => COLOR_GREEN,
                WatchStatus::Completed => COLOR_BLUE,
                WatchStatus::OnHold => COLOR_YELLOW,
                WatchStatus::Dropped => COLOR_RED,
                WatchStatus::PlanToWatch => COLOR_GRAY,
            };

            if entry.list_status.status != WatchStatus::PlanToWatch {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="3"/>"#,
                    ACTIVITY_X_TEXT_OFFSET + ACTIVITY_BAR_PADDING,
                    y_offset + 40 + ACTIVITY_BAR_PADDING,
                    (ACTIVITY_BAR_WIDTH as f32
                        * (entry.list_status.num_episodes_watched as f32
                            / entry.node.num_episodes as f32))
                        .min(ACTIVITY_BAR_WIDTH as f32)
                        - (2 * ACTIVITY_BAR_PADDING) as f32,
                    ACTIVITY_BAR_HEIGHT - 2 * ACTIVITY_BAR_PADDING,
                    color
                ));
            }

            let mut episodes = String::new();

            if entry.node.num_episodes > 0 {
                episodes.push_str(&format!(
                    " {}/{}",
                    entry.list_status.num_episodes_watched, entry.node.num_episodes
                ));
            }

            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="16" fill="white">{}{}</text>"#,
                ACTIVITY_X_TEXT_OFFSET,
                y_offset + 80,
                entry.list_status.status,
                episodes
            ));

            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="16" fill="white" text-anchor="end">{}</text>"#,
                ACTIVITY_WIDTH - ACTIVITY_PADDING,
                y_offset + 80,
                entry.list_status.updated_at.format("%b %d, %Y %H:%M %p").to_string(),
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}

impl ToSvg for MangaList {
    async fn to_svg(&self) -> String {
        let mut svg = String::new();

        let height =
            self.data.len() * (ACTIVITY_ENTRY_HEIGHT + ACTIVITY_PADDING) + ACTIVITY_PADDING;

        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
            ACTIVITY_WIDTH, height
        ));

        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="{}" rx="10"/>"#,
            ACTIVITY_WIDTH, height, COLOR_BACKGROUND
        ));

        for (index, entry) in self.data.iter().enumerate() {
            let y_offset = index * ACTIVITY_ENTRY_HEIGHT + (index + 1) * ACTIVITY_PADDING;

            if let Some(picture) = &entry.node.main_picture {
                if let Some(image) = fetch_image_base64(&picture.medium).await {
                    svg.push_str(&format!(
                        r#"<a href="https://myanimelist.net/manga/{}" target="_blank"><image x="{}" y="{}" width="{}" height="{}" href="data:image/png;base64,{}"/></a>"#,
                        entry.node.id, ACTIVITY_PADDING, y_offset, ACTIVITY_IMAGE_WIDTH, ACTIVITY_IMAGE_HEIGHT, image
                    ));
                }
            }

            let title = if entry.node.title.len() > ACTIVITY_TITLE_MAX_LENGTH {
                format!("{}...", &entry.node.title[..ACTIVITY_TITLE_MAX_LENGTH])
            } else {
                entry.node.title.clone()
            };

            svg.push_str(&format!(
                r#"<a href="https://myanimelist.net/manga/{}" target="_blank"><text x="{}" y="{}" font-family="Arial" font-size="20" fill="white">{}</text></a>"#,
                entry.node.id,
                ACTIVITY_X_TEXT_OFFSET,
                y_offset + 20,
                minimal_escape(title)
            ));

            if entry.list_status.score > 0 {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-family="Arial" font-size="16" fill="white" text-anchor="end">★ {}</text>"#,
                    ACTIVITY_WIDTH - ACTIVITY_PADDING,
                    y_offset + 20,
                    entry.list_status.score,
                ));
            }

            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="3"/>"#,
                ACTIVITY_X_TEXT_OFFSET,
                y_offset + 40,
                ACTIVITY_BAR_WIDTH,
                ACTIVITY_BAR_HEIGHT,
                COLOR_GRAY
            ));

            let color = match entry.list_status.status {
                ReadStatus::Reading => COLOR_GREEN,
                ReadStatus::Completed => COLOR_BLUE,
                ReadStatus::OnHold => COLOR_YELLOW,
                ReadStatus::Dropped => COLOR_RED,
                ReadStatus::PlanToRead => COLOR_GRAY,
            };

            if entry.list_status.status != ReadStatus::PlanToRead {
                let chapters = if entry.node.num_chapters > 0 {
                    entry.node.num_chapters
                } else {
                    if entry.list_status.num_chapters_read > 0 {
                        entry.list_status.num_chapters_read * 2
                    } else {
                        1
                    }
                };

                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="3"/>"#,
                    ACTIVITY_X_TEXT_OFFSET + ACTIVITY_BAR_PADDING,
                    y_offset + 40 + ACTIVITY_BAR_PADDING,
                    (ACTIVITY_BAR_WIDTH as f32
                        * (entry.list_status.num_chapters_read as f32 / chapters as f32))
                        .min(ACTIVITY_BAR_WIDTH as f32)
                        - (2 * ACTIVITY_BAR_PADDING) as f32,
                    ACTIVITY_BAR_HEIGHT - 2 * ACTIVITY_BAR_PADDING,
                    color
                ));
            }

            let chapters = if entry.node.num_chapters > 0 {
                entry.node.num_chapters.to_string()
            } else {
                "?".to_string()
            };

            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="16" fill="white">{} {}/{}</text>"#,
                ACTIVITY_X_TEXT_OFFSET,
                y_offset + 80,
                entry.list_status.status,
                entry.list_status.num_chapters_read,
                chapters,
            ));

            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="16" fill="white" text-anchor="end">{}</text>"#,
                ACTIVITY_WIDTH - ACTIVITY_PADDING,
                y_offset + 80,
                entry.list_status.updated_at.format("%b %d, %Y %H:%M %p").to_string(),
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}
