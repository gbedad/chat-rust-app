mod chat_message;
use chat_message::{ChatMessage};
fn main() {
    let message: ChatMessage = ChatMessage::new("Gerald".to_owned(), "Hello World!".to_owned());
    println!("{}", message.to_string());
}
