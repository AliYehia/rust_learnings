use crate::contact::{Contact, ContactTag};
use std::collections::HashMap;
use std::error::Error;

pub struct ContactManager {
    contacts: HashMap<String, Contact>, // name -> Contact
}

impl ContactManager {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }

    pub fn add_contact(&mut self, contact: Contact) -> Result<(), Box<dyn Error>> {
        if self.contacts.contains_key(&contact.name) {
            Err(format!("Contact {} already exists", contact.name))?
        } else {
            self.contacts.insert(contact.name.clone(), contact);
            Ok(())
        }
    }

    pub fn remove_contact(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if self.contacts.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("Contact {} not found", name))?
        }
    }

    pub fn find_contact(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }

    pub fn list_contacts_by_tag(&self, tag: ContactTag) -> Vec<&Contact> {
        self.contacts
            .values()
            .filter(|c| c.tag == tag)
            .collect()
    }

    pub fn update_email(&mut self, name: &str, new_email: &str) -> Result<(), Box<dyn Error>> {
        if let Some(contact) = self.contacts.get_mut(name) {
            contact.update_email(new_email);
            Ok(())
        } else {
            Err(format!("Contact {} not found", name))?
        }
    }

    pub fn all_contacts(&self) -> Vec<&Contact> {
        self.contacts.values().collect()
    }
}
