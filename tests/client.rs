use discord_rs::structs::client::{Client, GatewayIntents};


#[test]
fn create_client() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    let mut client = Client::new(&token, intents);
    let _ = client.connect();
}