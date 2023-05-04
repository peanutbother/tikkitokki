use anyhow::Result;
use application::{ApplicationError, Data};
use command::CommandError;
use poise::serenity_prelude as serenity;

mod api;
mod application;
mod command;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![command::embed()],
            skip_checks_for_owners: true,
            event_handler: |ctx, event, framework, data| {
                Box::pin(application::handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                log::info!("Syncing global commands");
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .map_err(CommandError::Register)?;
                log::info!("Userdata ready");
                Ok(Data {})
            })
        });

    log::info!("Initializing framework");
    framework.run().await.map_err(ApplicationError::Framework)?;
    Ok(())
}
