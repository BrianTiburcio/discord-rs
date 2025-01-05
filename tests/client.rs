use discord_rs::{
    gateway::GatewayIntents,
    client::{Client, EventHandler}
};

struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn on_ready(&self, _client: &mut Client) {
        println!("Bot is ready!");
    }
}

#[test]
fn create_client() {
    // Obtain your bot's token.
    let token = "YOUR BOT TOKEN HERE";
    
    // Define the intents your bot will have
    // Learn more about intents here
    // https://discord.com/developers/docs/events/gateway#list-of-intents
    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    // Create a mutable instance of your bot
    let mut _client = Client::new(Box::new(MyEventHandler), &token, intents).unwrap();
}