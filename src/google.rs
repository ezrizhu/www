use anyhow::Result;
use reqwest;

pub async fn ping() -> Result<()> {
    let cli = reqwest::Client::new();
    cli.get("https://www.google.com/ping")
        .query(&[("sitemap", "https://ericz.me/sitemap.xml")])
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
