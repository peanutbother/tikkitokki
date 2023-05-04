#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Failed to defer command: {0}")]
    Defer(#[source] poise::serenity_prelude::Error),
    #[error("Failed to reply to message: {0}")]
    Reply(#[source] poise::serenity_prelude::Error),
    #[error("Failed to reqister commands: {0}")]
    Register(#[source] poise::serenity_prelude::Error),
    #[error("An error occured: {0}")]
    Error(#[from] anyhow::Error),
}
