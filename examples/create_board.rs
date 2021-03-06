use std::net::{IpAddr, Ipv4Addr};
use trellobot::{Board, BoardConfig, TrelloBot};
use trellobot::{GenericBot, Host};

fn main() {
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
    let result = Board::create(
        BoardConfig::default().name("Trellobot test board"),
        &mut trellobot,
    )
    .unwrap();
    println!("result: {}", result);
}
