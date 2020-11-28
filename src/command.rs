use discord::model::Message;
use std::str::pattern::Pattern;
use std::str::Split;
/// used for processing discord message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Processor<'a> {
    name: &'a str,
    message: &'a mut Message,
}

impl<'a> Processor<'a> {
    pub fn new(message: &'a mut message, name: &'a str) -> Self {
        Self { message, name }
    }
    pub fn mentions_me(&self) -> bool {
        for mention in self.message.mentions.iter() {
            if mention.name == self.name {
                return true;
            }
        }
        return false;
    }
    pub fn split(&'a self, pat: P) -> Split<'a, P>
    where
        P: Pattern<'a>,
    {
        self.message.content.split(pat)
    }
}
