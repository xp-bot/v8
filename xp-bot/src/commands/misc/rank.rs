use serenity::{
    async_trait, builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction, prelude::Context,
};
use xp_db_connector::{guild_member::GuildMember, user::User, user_background::UserBackground};

use crate::{commands::XpCommand, utils::imggen::generate_ranking_card};

pub struct RankCommand;

#[async_trait]
impl XpCommand for RankCommand {
    fn name(&self) -> &'static str {
        "rank"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("rank")
            .description("Get info about your current level, badges and xp.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you want to check.")
                    .kind(serenity::model::application::command::CommandOptionType::User)
                    .required(false)
            })
    }

    async fn exec(&self, _ctx: &Context, command: &ApplicationCommandInteraction) {
        let mut user_id = command.user.id.0;

        match command.data.options.first() {
            Some(option) => {
                if let Some(user) = Some(option.value.as_ref().unwrap().clone()) {
                    user_id = user.as_str().unwrap().parse::<u64>().unwrap();
                }
            }
            None => {}
        }

        let user: User = User::from_id(user_id).await.unwrap();
        let guild_member: GuildMember = GuildMember::from_id(command.guild_id.unwrap().0, user_id)
            .await
            .unwrap();
        let background: Option<UserBackground> = match UserBackground::from_id(user_id).await {
            Ok(background) => Some(background),
            Err(_) => None,
        };

        log::info!("{:?}", user);
        log::info!("{:?}", guild_member);
        log::info!("{:?}", background);

        // TODO: pass user, guild_member and background to generate_ranking_card()
        let _ = generate_ranking_card();
    }
}
