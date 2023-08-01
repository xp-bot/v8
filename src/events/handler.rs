use serenity::{
    async_trait,
    model::prelude::Ready,
    prelude::{Context, EventHandler},
};
use log::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            info!("{} is connected on shard {}/{}!", ready.user.name, shard[0] + 1, shard[1]);
        } else {
            info!("Connected on shard");
        }
    }
}
