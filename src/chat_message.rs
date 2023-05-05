use chrono::{self, Local, Timelike};

fn hour_from_time(time: chrono::DateTime<Local>) -> String {
    return format!{"{hour}:{minute}", hour=time.hour(), minute=time.minute()};
}

pub struct ChatMessage {
    author: String,
    content: String,
    sent_time: String,
}

impl ChatMessage {
   pub fn new(author: String, content: String) -> ChatMessage {
        return ChatMessage{
            author,
            content,
            sent_time:hour_from_time(chrono::offset::Local::now()) ,
        };
    }
    pub fn to_string(&self) -> String {
        return format!("{author} on {sent_time}: {content}", author=self.author, sent_time=self.sent_time, content=self.content)
    }
}

