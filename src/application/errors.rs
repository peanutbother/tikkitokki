#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Framework error occured: {0}")]
    Framework(#[from] poise::serenity_prelude::Error),
    #[error("Command error occured: {0}")]
    Command(#[from] crate::command::CommandError),
}
