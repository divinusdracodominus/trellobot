use trellobot::{Card, CardList, Cards};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("cardlist.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let cards: Cards = serde_json::from_str(&contents).unwrap();
    let mut map = cards.build_id_map();
    for (k, v) in map.iter() {
        println!("card name: {}, id: {}", k, v);
    }
}
