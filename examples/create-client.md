```rs
use discord_rs::structs::client::{Client, GatewayIntents};

fn main() {
    // Obtain/Define your bot's token
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    // Define the intents your bot will have
    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    // Create a mutable instance of your bot
    let mut client = Client::new(&token, intents);
    // Connect your bot to the discord API
    let _ = client.connect();
}
```
