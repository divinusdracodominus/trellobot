#[macro_use]
extern crate tokio;
use discord::model::Event;
use discord::Discord;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};

use botnet::bot::{GenericBot, Host};
use botnet::trellobot::{Card, CardList, Cards, TrelloBot};
use botnet::command::{Processor, Command};

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

    let mut trellobot = Arc::new(Mutex::new(TrelloBot::from(bot)));

    let mut discord_bot =
        Discord::from_bot_token("")
            .unwrap();

    let (mut connection, _) = discord_bot.connect().expect("connection failed");
    let show_command = Command::new(&String::from("show"), |args| {
        if args.len() < 1 {
            return String::from("please provide more information in your request");
        }
        println!("args[0]: {}", args[0]);
        match args[0].as_str(){
            "cards" => {
                if args.len() < 2 {
                    String::from("please provide a board id")
                }else{
                    let mut bot = trellobot.lock().unwrap();
                    let card_list = bot.get_cards(&args[1]).unwrap();
                    let mut retstr = String::new();
                    for card in card_list.iter() {
                        retstr.push_str(&if let Some(desc) = &card.desc{
                            if desc != "" {
                                format!("name: {} \n description: {} \n", card.name, desc)
                            }else{
                                format!("name: {} \n", card.name)
                            }
                        }else {
                            format!("name: {} \n", card.name)
                        })
                    }
                    retstr
                }
            },
            _ => String::from("unrecognized command please try again"),
        }
    });
    let mut commands = Vec::new();
    commands.push(show_command);

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(mut message)) => {
                let mut processor = Processor::new(&mut message, "trellobot", &discord_bot, &commands);
                processor.process();
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
