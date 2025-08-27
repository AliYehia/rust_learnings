use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ContactTag {
    Friend,
    Family,
    Work,
}

#[derive(Clone)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub tag: ContactTag,
}

impl Contact {
    pub fn new(name: &str, phone: &str, email: &str, tag: ContactTag) -> Self {
        Self {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            tag,
        }
    }

    pub fn update_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }

    pub fn display(&self) {
        println!(
            "Name: {}, Phone: {}, Email: {}, Tag: {:?}",
            self.name, self.phone, self.email, self.tag
        );
    }
}

// Implement Display for nicer formatting
impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?}) - {} | {}", self.name, self.tag, self.phone, self.email)
    }
}
