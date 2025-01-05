use serde_json;
use discord_rs::models::{
    channel::Channel,
    snowflake::Snowflake
};
use std::thread;

// #[test]
// fn deserialize_channel() {
//     let channel_json = r#"{"id":"1120716431642865704","type":0,"last_message_id":"1224154872400842782","flags":0,"guild_id":"1118990126480097442","name":"testing","parent_id":"1118990127298007240","rate_limit_per_user":0,"topic":null,"position":4,"permission_overwrites":[{"id":"1118990126480097442","type":0,"allow":"0","deny":"1024"},{"id":"1119027280195362836","type":1,"allow":"1024","deny":"0"}],"nsfw":false}"#;
//     let channel = serde_json::from_str::<Channel>(&channel_json).unwrap();
//     println!("Manual deserialize Channel: {:#?}", channel);
// }

#[test]
fn deserialize_channel_threaded() {
    const STACK_SIZE_MB: usize = 16;
    const STACK_SIZE_BYTES: usize = STACK_SIZE_MB * 1_000_000;

    let builder = thread::Builder::new()
        // 128 MB stack size
        .stack_size(STACK_SIZE_BYTES);

    let _ = builder.spawn(|| {
        //let channel_json = r#"{"id":"1120716431642865704","type":0,"last_message_id":"1224154872400842782","flags":0,"guild_id":"1118990126480097442","name":"testing","parent_id":"1118990127298007240","rate_limit_per_user":0,"topic":null,"position":4,"permission_overwrites":[{"id":"1118990126480097442","type":0,"allow":"0","deny":"1024"},{"id":"1119027280195362836","type":1,"allow":"1024","deny":"0"}],"nsfw":false}"#;
        //let channel = Channel::new(&Snowflake::from("1120716431642865704".to_owned()));
        let channel_json = r#"{"id":"1120716431642865704","type":0,"last_message_id":"1224154872400842782","flags":0,"guild_id":"1118990126480097442","name":"testing","parent_id":"1118990127298007240","rate_limit_per_user":0,"topic":null,"position":4,"permission_overwrites":[{"id":"1118990126480097442","type":0,"allow":"0","deny":"1024"},{"id":"1119027280195362836","type":1,"allow":"1024","deny":"0"}],"nsfw":false}"#;
        let channel = serde_json::from_str::<Channel>(&channel_json).unwrap();
        
        println!("Manual deserialize Channel: {:#?}", channel);
    });
}