use std::error::Error;

use serenity::{async_trait, builder::CreateApplicationCommand, prelude::Context, model::prelude::application_command::ApplicationCommandInteraction};

pub mod misc;
pub mod admin;

#[async_trait]
pub trait XpCommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand;
    async fn exec(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub const COMMANDS: &[&dyn XpCommand] = &[
    &misc::about::AboutCommand,
    &misc::leaderboard::LeaderboardCommand,
    &misc::level::LevelCommand,
    &misc::rank::RankCommand,
    &misc::settings::SettingsCommand,
    &misc::voicetime::VoicetimeCommand,
    &admin::add::AddCommand,
    &admin::set::SetCommand,
    &admin::remove::RemoveCommand,
    &admin::reset::ResetCommand,
];
