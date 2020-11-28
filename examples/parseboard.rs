use botnet::trellobot::Board;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("board.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let board: Board = serde_json::from_str(&contents).unwrap();
    println!("board: {:?}", board);
}
