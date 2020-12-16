#[macro_use]
extern crate tokio;
use daemonize::Daemonize;
use discord::model::Event;
use discord::Discord;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};

use botnet::bot::{GenericBot, Host};
use botnet::commands::Command;
use botnet::trellobot::{Card, CardList, Cards, SimpleCard, TrelloBot};

// secret key that I probably shouldn't share, but don't care right now
//
// used for oath 1
fn test_query() {
    let host = Host {
        name: String::from("grunkle-stan"),
        ip: Some(IpAddr::V4(Ipv4Addr::new(10, 218, 201, 33))),
        domain: Some(String::from("kittenz.pdx.edu")),
    };
    

    let mut trellobot = Arc::new(Mutex::new(TrelloBot::from(bot)));

    let discord_bot =
        Discord::from_bot_token("")
            .unwrap();

    let (mut connection, _) = discord_bot.connect().expect("connection failed");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(mut message)) => {
                if Command::check_mentions(&message) {
                    let command = Command::handle(&message.content, trellobot.clone());
                    discord_bot.send_message(message.channel_id, &command, "", false);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}

fn switch_mode() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = File::create("/tmp/trellobot.out")?;
    let stderr = File::create("/tmp/trellobot.err")?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/trellobot.pid")
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop priviliges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }
    Ok(())
}

fn main() {
    //switch_mode().unwrap();
    test_query();
}
