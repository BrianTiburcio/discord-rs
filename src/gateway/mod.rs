pub mod socket;
pub mod enums;
pub mod types;

pub use enums::*;
use types::*;

use std::sync::mpsc;
use std::time::{Duration, Instant};
use serde_json::{json, Value};
use socket::Message;

use crate::{
    util::env::_get_client_token,
    models::{
        message,
        channel,
    }
};

/// The jitter to add to the heartbeat interval
const HEARTBEAT_JITTER: f32 = 0.1;

pub struct Gateway {}

impl Gateway {
    pub fn new(sender: &mpsc::Sender<(ExternalDispatchEvent, ExternalDispatchEventData)>, intents: u64) {
        let mut socket = socket::Socket::gateway();
        
        // Initialize variables used to maintain the socket connection
        let mut last_sequence = 0_usize;
        let mut interval = Duration::from_secs(5);
        let mut next_heartbeat = Instant::now();

        loop {
            // Attempt to get the next event from the socket
            let event = socket.read();

            // Most common error is a no-message
            // In this case, we should check if we need to send a heartbeat
            if event.is_err() {
                // TODO: Panic if its not a no-message errror
                let now = Instant::now();
                // This means not enough time has passed for us to send a heartbeat
                if next_heartbeat > now { continue; }

                // Get the heartbeat payload and send it through the socket
                let heartbeat = _get_heartbeat(last_sequence);
                let _ = socket.send(heartbeat);

                // Mark the next time a heartbeat should be sent
                next_heartbeat = now + interval;
                // Skip to reading the next event
                continue;
            }

            let message = event.unwrap();

            match message {
                // The socket connection has ended
                // break out of the loop
                Message::Close(_) => { break; },
                // Message::Binary(_) => {}
                // Message::Frame(_) => {},
                // Message::Ping(_) => {},
                // Message::Pong(_) => {},
                Message::Text(message) => {
                    last_sequence += 1;

                    let event = serde_json::from_str::<GatewayEventBody>(&message)
                        .expect("Failed to deserialize incoming data JSON");

                    // Match the event type
                    match GatewayEventIndexer[event.op] {
                        GatewayEvent::Dispatch => {
                            let dispatch_type = event.t.as_deref().map(|dispatch_type| DispatchEventIndexer[dispatch_type])
                                .expect("Failed to deserialize event type for dispatch event");

                            // Only inform the end user of dispatch events that they can handle
                            if let DispatchEvent::External(dispatch_type) = dispatch_type {
                                let dispatch_data = event.d.unwrap();
                                let dispatch_data = _parse_event_data(dispatch_type, dispatch_data);

                                // Allow the event to be handled by the end-user
                                let _ = sender.send((dispatch_type, dispatch_data));
                            }
                        },
                        GatewayEvent::Heartbeat => {
                            println!("Got heartbeat event: {:#?}", event);
                        },
                        GatewayEvent::Identify => {
                            println!("Got identify event: {:#?}", event);
                        },
                        GatewayEvent::PresenceUpdate => {
                            println!("Got presence update event: {:#?}", event);
                        },
                        GatewayEvent::VoiceStateUpdate => {
                            println!("Got voice update state event: {:#?}", event);
                        },
                        GatewayEvent::Resume => {
                            println!("Got resume event: {:#?}", event);
                        },
                        // Connection was likely dropped on discord's end. Mend it
                        GatewayEvent::Reconnect => {
                            let token = _get_client_token()
                                .expect("Could not get user token!");

                            // TODO: Create new connection
                            // Get and send the identify payload
                            // This allows to start receiving other events
                            let identify = _get_identify(&token, &intents);
                            socket.send(identify)
                                .expect("Failed to send identify payload");

                            panic!("Disconnected from the socket!");
                        },
                        GatewayEvent::RequestGuildMembers => {
                            println!("Got request guild members event: {:#?}", event);
                        },
                        GatewayEvent::InvalidSession => {
                            println!("Got invalid session event: {:#?}", event);
                        },
                        GatewayEvent::Hello => {
                            let token = _get_client_token()
                                .expect("Could not get user token!");

                            // Get and send the identify payload
                            // This allows to start receiving other events
                            let identify = _get_identify(&token, &intents);
                            socket.send(identify)
                                .expect("Failed to send identify payload");

                            // Set the interval and the next heartbeat
                            let data = event.d.unwrap();

                            if let Some(new_interval) = data.get("heartbeat_interval") {
                                let new_interval = new_interval.as_u64()
                                    .expect("Could not retrieve the interval from hello event");

                                interval = Duration::from_millis(new_interval);
                                // 0.25 is an arbitrarily chosen value meant to represent the jitter
                                // Since the jitter is only needed once, this is a better approach
                                // than using an entire library to create the suggested randomness
                                next_heartbeat = Instant::now() + Duration::from_millis(((new_interval as f32) * HEARTBEAT_JITTER) as u64);
                            }
                        },
                        GatewayEvent::HeartbeatAcknowledge => {
                            next_heartbeat = Instant::now() + interval;
                            //println!("Heartbeat acknowledged and reset");
                        },
                    };
                },
                _ => { break; }
            }
        }
    }
}

// Returns a heartbeat structure to send to Discord
fn _get_heartbeat(sequence: usize) -> socket::Message {
    let heartbeat = GatewayEventBody {
        op: GatewayEvent::Heartbeat as usize,
        d: Some(Value::Number(sequence.into())),
        s: None,
        t: None,
    };
    
    socket::Message::text(serde_json::to_string(&heartbeat).unwrap())
}

fn _get_identify(token: &String, intents: &u64) -> socket::Message {
    // Structure the initial identify request
    let identify = GatewayEventBody {
        op: GatewayEvent::Identify as usize,
        s: None,
        t: None,
        d: Some(json!({
            "token": token,
            "intents": intents,
            "properties": {
                "os": std::env::consts::OS,
                "browser": "discord-rs",
                "device": "discord-rs"
            }
        }))
    };

    // Serialize the identify request into JSON
    let identify = serde_json::to_string(&identify).unwrap();
    socket::Message::text(identify)
}

// Parses the event data from the dispatch event
fn _parse_event_data(event_type: ExternalDispatchEvent, data: Value) -> ExternalDispatchEventData {
    match event_type {
        // No data for the ready event
        ExternalDispatchEvent::Ready => ExternalDispatchEventData::None,
        // Message events
        ExternalDispatchEvent::MessageCreate
        | ExternalDispatchEvent::MessageDelete
        | ExternalDispatchEvent::MessageUpdate => {
            let message = serde_json::from_value::<message::Message>(data)
                .expect("Failed to parse message data from JSON");

            ExternalDispatchEventData::Message(message)
        },
        // Channel Events
        ExternalDispatchEvent::ChannelCreate
        | ExternalDispatchEvent::ChannelDelete
        | ExternalDispatchEvent::ChannelUpdate => {
            let channel = serde_json::from_value::<channel::Channel>(data)
                .expect("Failed to parse channel data from JSON");

                ExternalDispatchEventData::Channel(channel)
        },
        _ => ExternalDispatchEventData::None
    }
}