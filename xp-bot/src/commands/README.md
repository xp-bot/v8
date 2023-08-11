# Command structure
Every command should have the same 2 functions: `register()` for registering the command and `exec()` for executing the command.

```rs
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    
}
```

```rs
pub async fn exec(ctx: Context, command: ApplicationCommandInteraction) {

}
```
--- 
Additionally, commands should always follow the same structure for error handling and everything else. The following code is an example of a command that follows the structure.

```rs
pub async fn exec(ctx: Context, command: ApplicationCommandInteraction) {
    // ...

    let result = command
        .create_interaction_response(&ctx.http, |response| {
            // ...
        })
        .await;

    if let Err(why) = result {
        error!("Could not respond to command: {:?}", why);
    }
}
```