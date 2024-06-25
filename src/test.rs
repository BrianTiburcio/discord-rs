use dotenv;

use discord_rs::structs::client::{
    Client,
    GatewayIntents,
    ExternalDispatchEvent
};

use discord_rs::structs::{
    message::Message,
    locale::{Locale, Localizations},
    message_payload::MessagePayload,
    application_command::{ApplicationCommand, ApplicationCommandOption, ApplicationCommandOptionType}
};

pub fn main() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    let mut client = Client::new(&token, intents);

    let events = client.connect()
        .expect("Failed to login");
    
    loop {
        if let Ok((event_type, event_data)) = events.recv() {
            match event_type {
                ExternalDispatchEvent::Ready => {
                    //on_ready(&client);
                },
                ExternalDispatchEvent::MessageCreate => {
                    // NOTE: This currently breaks
                    on_message(&mut client, event_data.into());
                },
                _ => {}
            }
        }
    }
}

fn on_ready(
    _client: &mut Client,
) {
    println!("Received on_ready command");
    
    let command = ApplicationCommand::new("12345678987654321")
        .set_name("test-command")
        .set_description("A command that tests things")
        .set_dm_permission(false)
        .add_name_localizations(&[
            (Locale::Spanish, String::from("commando-prueba")),
            (Locale::French, String::from("commando-prueba"))
        ])
        .add_string_option(
            ApplicationCommandOption::new(ApplicationCommandOptionType::String)
                .set_name("favorite_number")
                .set_description("Your favorite number")
                .set_required(true)
                .add_choice("One", "1", None)
                .add_choice(
                    "Two",
                    "2",
                    Some(Localizations::from([
                        (Locale::Spanish, "Dos".to_string()),
                        (Locale::French, "Deux".to_string())
                    ]))
                )
        ).unwrap()
        .add_integer_option(
            ApplicationCommandOption::new(ApplicationCommandOptionType::Integer)
                .set_name("usertwo")
                .set_description("The user to test on")
                .set_required(true)
                .add_choice("One", 100, None)
        ).unwrap();

    println!("Calling register command: {:#?}", &command);
    // let _ = client.register_guild_commands(
    //     "1118990126480097442",
    //     &[command]
    // ).await;
}

fn on_message(
    client: &mut Client,
    message: Message
) {
    println!("{:?}: \"{}\"", message.author.global_name, message.content);

    if message.author.is_bot() { return; }

    let payload = MessagePayload::new()
        .set_content("Hello world");

    let channel = client.cache.get_channel(&message.channel_id.unwrap());
        
    if let Ok(channel) = channel {
        let _ = channel.send(payload);
    }

    // let _ = message.channel.send(payload);
    // message.send()
}

// async fn on_message(message: Message) {
//     if message.author.is_bot() { return; }
//     println!("Got message by {}: {}", message.author.username, message.content);
//     let _ = message.reply_content("Hello!").await;
//     //println!("Result: {:?}", res.unwrap());
// }