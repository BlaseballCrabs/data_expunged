use anyhow::{ensure, Result};
use db::Database;
use log::*;
use serde::Serialize;

pub mod db;
pub mod logger;
pub mod oauth_listener;

#[derive(Debug, Serialize)]
pub struct WebhookPayload<'a> {
    pub content: &'a str,
    pub avatar_url: &'static str,
}

async fn send_message(db: &Database, url: &str, content: &str) -> Result<()> {
    let hook = WebhookPayload {
        content,
        avatar_url: "http://hs.hiveswap.com/ezodiac/images/aspect_7.png",
    };
    let status = surf::post(url)
        .body(surf::Body::from_json(&hook).map_err(|x| x.into_inner())?)
        .send()
        .await
        .map_err(|x| x.into_inner())?
        .status();

    if status == surf::StatusCode::NotFound {
        debug!("webhook removed, deleting from database");
        db.remove_url(url).await?;
    } else {
        ensure!(status.is_success(), "Couldn't send webhook: {}", status);
    }

    Ok(())
}
