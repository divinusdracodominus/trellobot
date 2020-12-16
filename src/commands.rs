use crate::trellobot::{Board, Card, CardList, SimpleCard, TrelloBot, TrelloItem};
use discord::model::Message;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[structopt(name = "trellobot")]
pub struct Command {
    /// used with show command to get full text of a card
    #[structopt(short, long)]
    pub full: bool,
    /// the type of element being used
    #[structopt(short, long)]
    r#type: String,
    /// used for creating a card, this is the description on that card
    #[structopt(short, long)]
    pub description: Option<String>,
    /// due date of the card
    #[structopt(long)]
    due: Option<String>,
    #[structopt(short = "C", long)]
    create: bool,
    #[structopt(short = "S", long)]
    show: bool,
    #[structopt(short = "A", long)]
    attach: bool,
    /// the id of the element to operate on
    #[structopt(short, long)]
    id: Option<String>,
    /// the name of the element to operate on
    #[structopt(short, long)]
    name: Option<String>,
    /// move card, potential example @trellobot -M -t card,list -i "cardid","listid(target)"
    #[structopt(short = "M", long)]
    r#move: bool,
    /// search for an id by name
    #[structopt(short = "F", long)]
    find: bool,
}

impl Command {
    pub fn handle(text: &str, trellobot: Arc<Mutex<TrelloBot>>) -> String {
        match Self::from_iter_safe(text.split(' ')) {
            Ok(command) => {
                if command.show && command.create {
                    String::from("-C, and -S, cannot be used together")
                } else if command.show {
                    show(&command, trellobot)
                } else if command.create {
                    create(&command, trellobot)
                } else {
                    String::from("either -S, or -C must be used")
                }
            }
            Err(e) => e.message,
        }
    }
    pub fn check_mentions(message: &Message) -> bool {
        for user in &message.mentions {
            if user.name == "trellobot" && message.author.name != "trellobot" {
                return true;
            }
        }
        false
    }
}

fn show(_command: &Command, _trellobot: Arc<Mutex<TrelloBot>>) -> String {
    String::from("unimplemented")
}

fn create(_command: &Command, _trellobot: Arc<Mutex<TrelloBot>>) -> String {
    String::from("unimplemented")
}

fn attach(_command: &Command, _trellobot: Arc<Mutex<TrelloBot>>) -> String {
    String::from("unimplemented")
}