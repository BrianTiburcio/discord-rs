use discord_rs::structs::client::{Client, ExternalDispatchEvent, GatewayIntents};

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
    let events = client.connect().expect("Error connecting to gateway");

    if let Ok((event, data)) = events.recv() {
        match event {
            ExternalDispatchEvent::Ready => {
                println!("Ready event: {:?}", data);
            },
            _ => {}
        }
    }
}