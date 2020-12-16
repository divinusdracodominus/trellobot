use botnet::trellobot::Lists;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("lists.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lists: Lists = serde_json::from_str(&contents).unwrap();
    println!("{:?}", lists);
}