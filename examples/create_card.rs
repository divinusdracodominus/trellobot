use std::net::{IpAddr, Ipv4Addr};
use trellobot::{Card, TrelloBot};
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
    let mut card = Card::create(
        Card::basic(
            "basic card",
            "this card is to test my trellobot api",
            "10/10/2021",
            "5bee387b3bdaed643b3a69f8",
            None,
        ),
        &mut trellobot,
    )
    .unwrap();
    card.name = String::from("the card has now been updated");
    let card = card.update(&mut trellobot).unwrap();
    println!("card: {:?}", card);
    //println!("{}", card.delete(&mut trellobot).unwrap());
}
