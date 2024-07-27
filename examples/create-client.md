```rs
use discord_rs::structs::client::{Client, GatewayIntents, ExternalDispatchEvent};
use dotenv;

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
    let events = client.connect().expect("Error connecting to gateway");

    // Start listening to events from the websocket
    loop {
        if let Ok((event, _data)) = events.recv() {
            match event {
                ExternalDispatchEvent::Ready => {
                    println!("My bot is online!");
                },
                _ => {}
            }
        }
    }
}
```
