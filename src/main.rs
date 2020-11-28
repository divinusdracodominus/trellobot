#[macro_use]
extern crate tokio;
use discord::model::Event;
use discord::Discord;
use std::net::{IpAddr, Ipv4Addr};

use botnet::bot::{GenericBot, Host};
use botnet::trellobot::{Card, CardList, Cards, TrelloBot};

// secret key that I probably shouldn't share, but don't care right now
//
// used for oath 1
fn test_query() {
    let host = Host {
        name: String::from("grunkle-stan"),
        ip: Some(IpAddr::V4(Ipv4Addr::new(10, 218, 201, 33))),
        domain: Some(String::from("kittenz.pdx.edu")),
    };
    let bot: GenericBot = GenericBot::new(
        String::from("testbot"),
        Some(String::from("81e0add419e06edf04e46fac10f8e63d")),
        Some(String::from(
            "76b1f642507d11b398ef6d5187abf8f879e15df6a3465393cffffc123f954994",
        )),
        Some(host),
    );

    let mut trellobot = TrelloBot::from(bot);

    let mut discord_bot =
        Discord::from_bot_token("")
            .unwrap();

    let (mut connection, _) = discord.connect().expect("connection failed");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                for mention in message.mentions.iter() {
                    if mention.name == String::from("trellobot") {
                        // the bot was mentioned
                    }
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

fn main() {
    test_query();
}
