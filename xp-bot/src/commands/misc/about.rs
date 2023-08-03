use log::error;
use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType, component::ButtonStyle, ReactionType},
    prelude::Context,
};

use crate::utils::colors;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("about")
        .description("Get every info about the bot.")
}

pub async fn exec(ctx: Context, command: ApplicationCommandInteraction) {
    let result = command.create_interaction_response(&ctx.http, |response| {
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
                        .footer(|footer| footer.text(format!("© 2020-2023 namespace.media - Shard {}", ctx.shard_id + 1)))
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
    }).await;

    if let Err(why) = result {
        error!("Could not respond to command: {:?}", why);
    }
}
