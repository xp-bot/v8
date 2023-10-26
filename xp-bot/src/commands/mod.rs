use std::error::Error;

use serenity::{
    async_trait, builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction, prelude::Context,
};

pub mod admin;
pub mod games;
pub mod misc;

#[async_trait]
pub trait XpCommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand;
    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub const COMMANDS: &[&dyn XpCommand] = &[
    &misc::about::AboutCommand,
    &misc::leaderboard::LeaderboardCommand,
    &misc::level::LevelCommand,
    &misc::rank::RankCommand,
    &misc::settings::SettingsCommand,
    &misc::voicetime::VoicetimeCommand,
    &misc::incognito::IncognitoCommand,
    &misc::distance::DistanceCommand,
    &admin::add::AddCommand,
    &admin::set::SetCommand,
    &admin::remove::RemoveCommand,
    &admin::reset::ResetCommand,
    &admin::setlevel::SetLevelCommand,
    &admin::setstreak::SetStreakCommand,
    &games::fish::FishCommand,
    &games::roll::RollCommand,
    &games::loot::LootCommand,
    &games::daily::DailyCommand,
    &games::trivia::TriviaCommand,
    &games::party::PartyCommand,
];