use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            println!("{} is connected on shard {}/{}!", ready.user.name, shard[0] + 1, shard[1]);
        } else {
            println!("Connected on shard");
        }
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        println!("{}: {}", msg.author.name, msg.content);
        /*         if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        } */
    }
}
