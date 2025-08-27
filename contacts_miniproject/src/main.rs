mod contact;
mod manager;

use crate::contact::{Contact, ContactTag};
use crate::manager::ContactManager;
use std::fs;
use std::io::{self, Write};

fn main() {
    let mut manager = ContactManager::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut args = input.split_whitespace();
        match args.next() {
            Some("add") => {
                let name = args.next();
                let phone = args.next();
                let email = args.next();
                let tag = args.next();
                if let (Some(name), Some(phone), Some(email), Some(tag)) = (name, phone, email, tag) {
                    let tag_enum = match tag {
                        "Friend" => ContactTag::Friend,
                        "Family" => ContactTag::Family,
                        "Work" => ContactTag::Work,
                        _ => {
                            println!("Unknown tag, use Friend/Family/Work");
                            continue;
                        }
                    };
                    let contact = Contact::new(name, phone, email, tag_enum);
                    if let Err(e) = manager.add_contact(contact) {
                        println!("Error adding contact: {}", e);
                    }
                } else {
                    println!("Usage: add <name> <phone> <email> <tag>");
                }
            }
            Some("list") => {
                for contact in manager.all_contacts() {
                    println!("{}", contact);
                }
            }
            // example of usage of list_contacts_by_tag
            Some("list-friends") => {
                let friends_vec = manager.list_contacts_by_tag(ContactTag::Friend);
                for friend in friends_vec {
                    println!("{}", friend);
                }
            }
            Some("find") => {
                if let Some(name) = args.next() {
                    match manager.find_contact(name) {
                        Some(c) => println!("{}", c),
                        None => println!("Contact not found"),
                    }
                } else {
                    println!("Usage: find <name>");
                }
            }
            Some("update_email") => {
                if let (Some(name), Some(new_email)) = (args.next(), args.next()) {
                    if let Err(e) = manager.update_email(name, new_email) {
                        println!("Error updating email: {}", e);
                    }
                } else {
                    println!("Usage: update_email <name> <new_email>");
                }
            }
            Some("remove") => {
                if let Some(name) = args.next() {
                    if let Err(e) = manager.remove_contact(name) {
                        println!("Error removing contact: {}", e);
                    }
                } else {
                    println!("Usage: remove <name>");
                }
            }
            Some("exit") => break,
            _ => println!("Commands: add, list, find, update_email, remove, exit"),
        }
    }
}
