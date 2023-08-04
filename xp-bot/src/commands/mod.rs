use serenity::{async_trait, builder::CreateApplicationCommand, prelude::Context, model::prelude::application_command::ApplicationCommandInteraction};

pub mod misc;

#[async_trait]
pub trait XpCommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand;
    async fn exec(&self, ctx: &Context, command: &ApplicationCommandInteraction);
}

pub const COMMANDS: &[&dyn XpCommand] = &[
    &misc::about::AboutCommand,
    &misc::leaderboard::LeaderboardCommand,
    &misc::level::LevelCommand,
    &misc::rank::RankCommand,
    &misc::settings::SettingsCommand,
];
