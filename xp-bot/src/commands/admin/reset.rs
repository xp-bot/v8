use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        self,
        prelude::{
            application_command::ApplicationCommandInteraction, command::CommandOptionType,
            InteractionResponseType,
        },
        Permissions,
    },
    prelude::Context,
};

use crate::commands::XpCommand;

pub struct ResetCommand;

#[async_trait]
impl XpCommand for ResetCommand {
    fn name(&self) -> &'static str {
        "reset"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("reset")
            .description("Reset settings and users.")
            .create_option(|option| {
                option
                    .kind(CommandOptionType::SubCommand)
                    .name("user")
                    .description("Reset xp of a specified user.")
                    .create_sub_option(|sub_option| {
                        sub_option
                            .kind(CommandOptionType::User)
                            .name("user")
                            .description("User to reset xp of.")
                            .required(true)
                    })
            })
            .create_option(|option| {
                option
                    .kind(CommandOptionType::SubCommand)
                    .name("community")
                    .description("Reset community settings.")
                    .create_sub_option(|sub_option| {
                        sub_option
                            .kind(CommandOptionType::String)
                            .add_string_choice("Settings", "settings")
                            .add_string_choice("XP", "xp")
                            .name("type")
                            .description("Type to reset.")
                            .required(true)
                    })
            })
            .default_member_permissions(Permissions::MANAGE_GUILD)
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let first_option = command.data.options.first().unwrap();

        /*
           events are handled in /events/handler.rs
        */

        match first_option.kind {
            CommandOptionType::SubCommand => match first_option.name.as_str() {
                "user" => {
                    let second_option = first_option.options.first().unwrap();

                    match second_option.name.as_str() {
                        "user" => {
                            command
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::Modal)
                                        .interaction_response_data(|message| {
                                            message.title("Reset user xp");
                                            message.custom_id("reset_user_xp");
                                            message.components(|components| {
                                                components.create_action_row(|action_row| {
                                                    action_row.create_input_text(|input_text| {
                                                        input_text
                                                            .placeholder("Type 'reset' to confirm.")
                                                            .style(model::application::component::InputTextStyle::Short)
                                                            .label("This action is irreversible.")
                                                            .custom_id(
                                                               format!("reset_user_xp_input_{}", command.data.options.first().unwrap().options.first().unwrap().value.as_ref().unwrap().as_str().unwrap())
                                                            )
                                                    })
                                                })
                                            })
                                        })
                                })
                                .await?;
                        }
                        _ => {}
                    }
                }
                "community" => {
                    let second_option = first_option.options.first().unwrap();

                    match second_option.value.as_ref().unwrap().as_str().unwrap() {
                        "settings" => {
                            command
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::Modal)
                                        .interaction_response_data(|message| {
                                            message.title("Reset community settings");
                                            message.custom_id("reset_community_settings");
                                            message.components(|components| {
                                                components.create_action_row(|action_row| {
                                                    action_row.create_input_text(|input_text| {
                                                        input_text
                                                            .placeholder("Type 'reset' to confirm.")
                                                            .style(model::application::component::InputTextStyle::Short)
                                                            .label("This action is irreversible.")
                                                            .custom_id(
                                                               format!("reset_community_settings_input")
                                                            )
                                                    })
                                                })
                                            })
                                        })
                                })
                                .await?;
                        }
                        "xp" => {
                            command
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::Modal)
                                        .interaction_response_data(|message| {
                                            message.title("Reset community xp");
                                            message.custom_id("reset_community_xp");
                                            message.components(|components| {
                                                components.create_action_row(|action_row| {
                                                    action_row.create_input_text(|input_text| {
                                                        input_text
                                                            .placeholder("Type 'reset' to confirm.")
                                                            .style(model::application::component::InputTextStyle::Short)
                                                            .label("This action is irreversible.")
                                                            .custom_id(
                                                               format!("reset_community_xp_input")
                                                            )
                                                    })
                                                })
                                            })
                                        })
                                })
                                .await?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}
