use crate::application::Context;
use anyhow::Result;

mod errors;
pub use errors::CommandError;

/// Fetches the video embed url for the specified tiktok url
#[poise::command(slash_command)]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "url of the tiktok video"] url: String,
) -> Result<(), CommandError> {
    log::debug!("fetch requested: {}", url);
    ctx.defer().await.map_err(CommandError::Defer)?;

    async fn error(ctx: &Context<'_>, err: anyhow::Error) -> Result<(), CommandError> {
        log::error!("fetch failed: {}", err);
        ctx.send(|b| b.embed(|e| e.title("Error!").description(format!("{err}"))))
            .await
            .map_err(CommandError::Reply)?;
        Ok(())
    }

    match crate::api::get_id(url.clone()).await {
        Ok(id) => {
            log::debug!("matched {id} with {}", &url);
            match crate::api::request_url(id).await {
                Ok(url) => {
                    log::debug!("fetch returned: {}", url);
                    ctx.send(|b| b.content(url))
                        .await
                        .map_err(CommandError::Reply)?;
                }
                Err(err) => error(&ctx, err.into()).await?,
            }
        }
        Err(err) => error(&ctx, err.into()).await?,
    }

    Ok(())
}
