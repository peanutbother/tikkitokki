use crate::{api, command::CommandError};
use anyhow::{anyhow, Result};
use poise::serenity_prelude as serenity;
use regex::Regex;
use std::collections::HashSet;

mod errors;
pub use errors::ApplicationError;

#[derive(Debug)]
pub struct Data {}

pub type Context<'a> = poise::Context<'a, Data, CommandError>;

pub async fn handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, CommandError>,
    _data: &Data,
) -> Result<(), CommandError> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r#"https://.*\.?tiktok\.com/@.+/video/([A-z0-9]+)"#).unwrap();
        static ref RE2: Regex = Regex::new(r#"https://.*\.?tiktok\.com/([A-z0-9]+)/"#).unwrap();
    }
    match event {
        poise::Event::Message { new_message } => {
            let mut urls: HashSet<String> = HashSet::new();

            for url in RE
                .find_iter(&new_message.content)
                .chain(RE2.find_iter(&new_message.content))
                .map(|m| m.as_str())
            {
                // resolve urls and request direct links
                if let Ok(id) = api::get_id(url.to_owned()).await {
                    if let Ok(value) = api::request_url(id).await {
                        urls.insert(value);
                    }
                }
            }

            // if urls were matched and successfully fetched post them as reply
            if !urls.is_empty() {
                new_message
                    .reply(
                        ctx,
                        urls.iter()
                            .map(|url| format!("{url}\n"))
                            .collect::<String>(),
                    )
                    .await
                    .map_err(|e| CommandError::Error(anyhow!("{e}")))?;
            }
        }
        _ => {}
    }
    Ok(())
}
