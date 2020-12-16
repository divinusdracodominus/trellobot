use structopt::StructOpt;
use discord::model::Message;
use std::sync::{Arc, Mutex};
use crate::trellobot::{TrelloBot, CardList, SimpleCard, Card, TrelloItem, Board};

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[structopt(name = "trellobot")]
pub struct Command{
    /// used with show command to get full text of a card
    #[structopt(short, long)]
    pub full: bool,
    /// id of the board to use
    #[structopt(short, long)]
    pub board: Option<String>,
    /// specify list by id
    #[structopt(long)]
    pub list_id: Option<String>,
    #[structopt(long)]
    pub card_id: Option<String>,
     /// id of the card to search for
    #[structopt(short, long)]
    pub card: Option<String>,
    /// id of the list to add to or display cards from
    #[structopt(short, long)]
    pub list: Option<String>,
    /// list the lists on a board
    #[structopt(short, long)]
    pub get_lists: Option<String>,
    /// used for creating a card, this is the description on that card
    #[structopt(short, long)]
    pub description: Option<String>,
    #[structopt(long)]
    pub create_card: Option<String>,
    /// due date of the card
    #[structopt(long)]
    due: Option<String>,
    #[structopt(short = "C", long)]
    create: bool,
    #[structopt(short = "S", long)]
    show: bool,
}

impl Command {
    pub fn handle(text: &str, trellobot: Arc<Mutex<TrelloBot>>) -> String{
        match Self::from_iter_safe(text.split(" ")) {
            Ok(command) => {
                if command.show && command.create {
                    String::from("-C, and -S, cannot be used together")
                }
                else if command.show {
                    show(&command, trellobot)
                }
                else if command.create{
                    create(&command, trellobot)
                }else{
                    String::from("either -S, or -C must be used")
                }
            },
            Err(e) => e.message,
        }
    }
    pub fn check_mentions(message: &Message) -> bool{
        for user in &message.mentions {
            if user.name == "trellobot" && message.author.name != "trellobot" {
                return true;
            }
        }
        false
    }
}

fn show(command: &Command, trellobot: Arc<Mutex<TrelloBot>>) -> String {
    if let Some(board) = &command.board {
        let mut bot = trellobot.lock().unwrap();
        match Board::get_cards(&board, &mut bot) {
            Ok(cards) => {
                if command.full {
                    return format!("{:?}", cards);
                }
                return format!("{:?}", cards.simplify());
            },
            Err(e) => {
                return format!("error: {}", e);
            }
        }
    }
    if let Some(card) = &command.card {
        let mut bot = trellobot.lock().unwrap();
        match Card::get(&card, &mut bot) {
            Ok(card) => {
                if command.full {
                    return format!("{:?}", card);
                }
                return SimpleCard::from(&card).ascii_fmt();
            },
            Err(e) => {
                return format!("error: {}", e);
            }
        }
    }
    String::from("unrecognized option configuration")
}

fn create(command: &Command, trellobot: Arc<Mutex<TrelloBot>>) -> String {
    String::from("unimplemented")
}