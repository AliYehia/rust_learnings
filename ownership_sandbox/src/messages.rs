pub fn run_messages_demo() {
    let mut recieved_messages: Vec<String> = Vec::new();
    let mut sender_messages: Vec<String> = Vec::new();

    let message1 = String::from("What");
    let message2 = String::from("is");
    let message3 = String::from("hot");

    sender_ownership(&mut sender_messages, message1);
    // println!("Message1 is gone {}", message1); message1 is moved, this will fail

    reciever_ownership(&mut recieved_messages, message2);

    let message4 = deliver_and_copy(&mut sender_messages, message3);
    reciever_ownership(&mut recieved_messages, message4);

    let last_message = check_latest_message(&mut recieved_messages);
    println!("Last message reciever got was: {}", last_message);
}

fn sender_ownership(sender: &mut Vec<String>, message: String) {
    println!("Pushing message - {} - to senders messages", message);
    sender.push(message);
}

fn reciever_ownership(reciever: &mut Vec<String>, message: String) {
    println!("Delivering message - {} - to reciever messages", message);
    reciever.push(message);
}

fn deliver_and_copy(reciever: &mut Vec<String>, message: String) -> String {
    println!("Delivering message - {} - to reciever messages. Will also copy", message);
    let new_message = message.clone();
    reciever.push(message);
    new_message
}

fn check_latest_message(reciever: &mut Vec<String>) -> String {
    let latest_message = reciever.pop();
    latest_message.unwrap()
}