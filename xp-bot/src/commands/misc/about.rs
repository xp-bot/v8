use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{
        application_command::ApplicationCommandInteraction, component::ButtonStyle,
        InteractionResponseType, ReactionType,
    },
    prelude::Context,
};

use crate::{commands::XpCommand, utils::colors};

pub struct AboutCommand;

#[async_trait]
impl XpCommand for AboutCommand {
    fn name(&self) -> &'static str {
        "about"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("about")
            .description("Get every info about the bot.")
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let time_then = std::time::Instant::now();
        let _ = ctx.http.get_gateway().await?;
        let latency = time_then.elapsed().as_millis();

        let _ = command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(
                        |embed: &mut CreateEmbed| embed
                            .title("Reimagine your Community")
                            .description("Elevate your Discord community to the next level with top-tier leveling, endless customizability and more.\n\nBuilt with [serenity](https://github.com/serenity-rs/serenity) and [rust](https://www.rust-lang.org/).")
                            .field("Official Support Server", "[discord.gg](https://discord.xp-bot.net)", true)
                            .field("Vote", "[top.gg](https://vote.xp-bot.net)", true)
                            .field("Status", "[status](https://status.xp-bot.net)", true)
                            .footer(|footer| footer.text(format!("© 2020-2023 namespace.media - Shard {} - {}ms", ctx.shard_id + 1, latency)))
                            .colour(colors::blue())
    
                    ).components(
                        |components| components
                            .create_action_row(|action_row| action_row
                                .create_button(|button| button
                                    .label("Server Dashboard")
                                    .style(ButtonStyle::Link)
                                    .emoji(ReactionType::Unicode("🛠️".to_string()))
                                    .url(format!("https://xp-bot.net/servers/{}", &command.guild_id.unwrap().to_string()))
                                )
                                .create_button(|button| button
                                    .label("Account Settings")
                                    .style(ButtonStyle::Link)
                                    .emoji(ReactionType::Unicode("🙋".to_string()))
                                    .url("https://xp-bot.net/me")
                                )
                                .create_button(|button| button
                                    .label("Premium")
                                    .style(ButtonStyle::Link)
                                    .emoji(ReactionType::Unicode("👑".to_string()))
                                    .url("https://xp-bot.net/premium")
                                )
                                .create_button(|button| button
                                    .label("Privacy Policy")
                                    .style(ButtonStyle::Link)
                                    .emoji(ReactionType::Unicode("🔖".to_string()))
                                    .url("https://xp-bot.net/legal/privacy")
                                )
                            )
                    )
                })
        }).await?;

        Ok(())
    }
}
