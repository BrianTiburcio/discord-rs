use crate::client::{
    Client,
    ExternalDispatchEventData,
    ExternalDispatchEvent
};

use crate::models::message::Message;

pub trait Handler {
    fn handle_event(&mut self, client: &mut Client, ) {
        // Start listening to events from the websocket
        // loop {
        //     if let Ok(event) = events.recv() {
        //         match event {
        //             (ExternalDispatchEvent::Ready, _) => {
        //                 println!("My bot is online!");
        //             },
        //             (ExternalDispatchEvent::MessageCreate, Some(ExternalDispatchEventData::Message(message)) ) => {
        //                 //handle_message(&mut client, message);

        //                 // if message.author.is_bot() { continue; }

        //                 // let channel = message.get_channel().unwrap();
        //                 // let response = MessagePayload {
        //                 //     content: Some("This is a test!".to_string()),
        //                 //     ..Default::default()
        //                 // };

        //                 // channel.send(response).expect("Error sending message");
        //             },
        //             _ => {}
        //         }
        //     }
        // }
    }

    fn handle_message(&self, _message: Message) {}
    fn handle_ready(&self, _client: &mut Client) {}
}