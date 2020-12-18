use trellobot::{Card, CardList, Cards};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr};
use trellobot::{Member, TrelloBot};
use trellobot::{GenericBot, Host};

fn main() {
    let mut file = File::open("memberlist.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let members: Vec<Member> = serde_json::from_str(&contents).unwrap();

    println!("members: {:?}", members);
    //println!("{:?}", members);
}