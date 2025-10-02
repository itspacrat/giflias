use std::env;
use ez_ffmpeg::{FfmpegContext, FfmpegScheduler};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const MSG_PREFIX: &str = ".g";

struct Handler;

// return file path?
// use the ffmpeg context builder in here maybe
pub fn ffmpeg_conv_to_gif() {

}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        let mut gif_builder = FfmpegContext::builder();
        let mut out: String = format!("");
        let msg_args = msg.content.split(" ").collect::<Vec<_>>();

        if msg_args[0] == MSG_PREFIX {
            match msg_args[1] {
                //"c" => { /* check if this is a reply to an image; if not do nothing */}
                //"a" => { /* add this to an alias group*/}
                "help" => {
                    out = format!("```diff\n- help menu in progress... come back later! -\n```")
                }
                "c"|_ => {
                    out = format!(
                        "inacessible or unknown command;\n type `.g help` to see available commands :3"
                    )
                }
            }
            if let Err(why) = msg.reply_ping(&ctx.http, out.as_str()).await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
