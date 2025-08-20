fn main() {
    struct Book {
        title: String,
        pages: i32,
        available: bool
    }

    impl Book {
        fn print_title(&self) {
            println!("Book title is: {}, and has {} pages", self.title, self.pages);
        }
        fn checkout(&mut self) {
            if self.available {
                self.available = false;
                println!("Book {} is now checked out", self.title);
            } else {
                println!("Book {} was already checked out", self.title);
            }
            self.available = false;
        }
    }

    let mut book1 = Book {
        title: String::from("The Rust Programming Language"),
        pages: 550,
        available: true
    };

    book1.checkout();
    book1.checkout();

    book1.print_title();
}
