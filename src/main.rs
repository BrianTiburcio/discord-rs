use discord_rs::structs::{
    client::{Client, GatewayIntents},
    snowflake::Snowflake
};

fn main() {
    #[cfg(test)]
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    #[cfg(not(test))]
    let token = "MTExOTAyNzI4MDE5NTM2MjgzNg.GAW4Mb.UgzTyOdhVZqLA5j5N9jm0EDx_W6XemUYGYc7xs";

    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    let mut client = Client::new(&token, intents);
    let _ = client.connect();
}