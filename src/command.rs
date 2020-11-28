use discord::model::Message;
use discord::Discord;
use std::str::Split;
pub struct Command<F: Fn(&Vec<String>) -> String> {
    name: String,
    func: F,
}
impl<F: Fn(&Vec<String>) -> String> Command<F> {
    pub fn new(name: &str, func: F) -> Self {
        Self {name: name.to_string(), func}
    }
    pub fn execute(&self, args: &Vec<String>) -> String {
        // tricking the compiler, self.func(args), is technically valid
        // but compiler can't distinguish between an associated method, and a field that is a function pointer
        // because all rust associated methods, are really function pointers
        let func = &self.func;
        func(args)
    }
}

impl<F: Fn(&Vec<String>) -> String> PartialEq<Command<F>> for Command<F> {
    fn eq(&self, other: &Command<F>) -> bool{
        self.name == other.name
    }
}


/// used for processing discord message
pub struct Processor<'a, F: Fn(&Vec<String>) -> String> {
    name: &'a str,
    message: &'a mut Message,
    bot: &'a Discord,
    commands: &'a Vec<Command<F>>,
}

impl<'a, F: Fn(&Vec<String>) -> String> Processor<'a, F> {
    pub fn new(message: &'a mut Message, name: &'a str, bot: &'a Discord, commands: &'a Vec<Command<F>>) -> Self {
        Self { message, name, bot, commands }
    }
    pub fn mentions_me(&self) -> bool {
        for mention in self.message.mentions.iter() {
            if mention.name == self.name {
                return true;
            }
        }
        return false;
    }
    pub fn split(&'a self, pat: &'a str) -> Split<'a, &str>
    {
        self.message.content.split(pat)
    }
    pub fn process(&mut self) {
        if self.mentions_me() {
            let command_args: Vec<String> = self.split(" ").map(|s| {s.to_string()}).collect();
            if let Some(arg) = command_args.get(1) {
                println!("arg before match: {}", arg);
                match search_commands(&self.commands, arg.to_string()){
                    Some(index) => {
                        println!("command_args values: {:?}", command_args);
                        let len = command_args.len();
                        let msg = self.commands[index].execute(&command_args[2..len].to_vec());
                        println!("message after function: {}", msg);
                        self.bot.send_message(self.message.channel_id, &msg, "", false).unwrap();
                    },
                    None => {
                        self.bot.send_message(self.message.channel_id, "command not found please try again", "", false).unwrap();
                    },
                }
            }else {
                self.bot.send_message(self.message.channel_id, "in sufficient commands provided", "", false).unwrap();
            }
        }
    }
}

pub fn search_commands<F: Fn(&Vec<String>) -> String>(commands: &Vec<Command<F>>, key: String) -> Option<usize>{
    let mut index = 0;
    for cmd in commands.iter() {
        println!("{:?}, arg value: {}", cmd.name, key);
        if cmd.name == key {
            return Some(index);
        }
        index += 1;
    }
    None
}