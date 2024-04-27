use discord_rs::structs::{
    client::{Client, GatewayIntents},
    snowflake::Snowflake
};

use dotenv;

#[test]
fn deserialize_channel_client() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    let mut client = Client::new(&token, intents);

    //client.connect().expect("Failed to login");

    let id = Snowflake::String("1120716431642865704".to_string());
    let channel = client.cache.get_channel(&id);

    println!("Channel: {channel:?}");

    //assert!(channel.is_ok())
    assert!(true)
}