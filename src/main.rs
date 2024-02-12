use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use futures::stream::{StreamExt, TryStreamExt};
use misskey::{prelude::*, HttpClient, TimelineRange};
use url::Url;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, env, hide_env_values = true)]
    url: Url,
    #[arg(long, env, hide_env_values = true)]
    api_token: String,
    #[arg(long, env, hide_env_values = true)]
    x_cisskey_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Args::parse();

    // Create `HttpClient` with token `opt.i`
    let client = HttpClient::builder(opt.url)
        .token(opt.api_token)
        .header("X-Cisskey-Key", opt.x_cisskey_key)
        .build()?;

    // Get the latest note
    let latest_note = client
        .user_notes(&client.me().await?.id, TimelineRange::Unbounded)
        .take(1)
        .try_next()
        .await?
        .ok_or(anyhow::anyhow!("No note found"))?;

    let delta = Utc::now() - latest_note.created_at;
    if delta.num_hours() < 24 {
        return Ok(());
    }

    // Create a note if the user has not posted for more than 24 hours
    client
        .create_note(format!(
            "ユーザが最後に投稿してから{}時間以上経過しました。",
            delta.num_hours()
        ))
        .await?;

    Ok(())
}
