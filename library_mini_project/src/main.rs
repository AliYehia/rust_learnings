fn main() {
    enum Message {
        Text(String),
        Image(String),
        Video(String),
    }

    struct Book {
        title: String,
        pages: i32,
        availabile: bool,
    }

    impl Book {
        fn checkout(&mut self) {
            if !self.availabile {
                println!("Book already checked out");
            } else {
                self.availabile = false;
                println!("Checked out the book {}", self.title);
            }
        }

        fn return_book(&mut self) {
            if !self.availabile {
                self.availabile = true;
                println!("Returned the book {}", self.title);
            } else {
                println!("Book is already in the library?")
            }
        }
    }

    struct Users {
        name: String,
        borrowed_books: Vec<String>,
        messages: Vec<Message>,
    }

    impl Users {
        fn borrow_book(&mut self, book: &mut Book) {
            println!("User {} will try to borrow book {}", self.name, book.title);

            let was_available = book.availabile;
            book.checkout();

            // If the checkout actually changed availability, record it and notify
            if was_available && !book.availabile {
                self.borrowed_books.push(book.title.clone());
                self.messages
                    .push(Message::Text(format!("You borrowed '{}'", book.title)));
            }
        }

        fn return_book(&mut self, book: &mut Book) {
            println!("User {} will return the book {}", self.name, book.title);

            let was_borrowed = !book.availabile;
            book.return_book();

            // If the return actually changed availability, remove from list and notify
            if was_borrowed && book.availabile {
                // cute way to remove the book title from borrowed_books
                self.borrowed_books.retain(|t| t != &book.title);
                self.messages
                    .push(Message::Text(format!("You returned '{}'", book.title)));
            }
        }

        fn print_messages(&self) {
            println!("--- Messages for {} ---", self.name);
            for (i, msg) in self.messages.iter().enumerate() {
                match msg {
                    Message::Text(s) => println!("{}: {}", i + 1, s),
                    Message::Image(img) => println!("{}: [image] {}", i + 1, img),
                    Message::Video(vid) => println!("{}: [video] {}", i + 1, vid),
                }
            }
            println!();
        }

        fn list_borrowed(&self) {
            println!("{} has borrowed: {:?}", self.name, self.borrowed_books);
        }
    }

    fn find_book(library: &Vec<Book>, book_name: &str) {
        for book in library {
            if book.title == book_name {
                println!("Book found in library");
                return
            }
        }
        println!("Book not found in library");
    }

    fn list_books_in_library(library: &Vec<Book>) {
        for book in library {
            println!("Book {} in library", book.title);
        }
    }

    fn list_available_books_in_library(library: &Vec<Book>) {
        for book in library {
            if book.availabile {
                println!("Book {} in library - and it's available", book.title);
            } else {
                println!("Book {} in library - but it's unavailable", book.title);
            }
        }
    }

    fn show_first_n_books(library: &Vec<Book>, n: usize) {
        if n > library.len() {
            println!("There are only {} books in the library", library.len());
        } else {
            for book in &library[..n] {
                println!("Book {} in library", book.title);
            }
        }
    }

    let mut library: Vec<Book> = Vec::new();

     let book1 = Book {
        title: String::from("The Rust Programming Language"),
        pages: 550,
        availabile: true,
    };

    let book2 = Book {
        title: String::from("The C Programming Language"),
        pages: 300,
        availabile: true,
    };

    let book3 = Book {
        title: String::from("The Go Programming Language"),
        pages: 400,
        availabile: true,
    };

    let mut user1 = Users {
        name: String::from("Ali"),
        borrowed_books: Vec::new(),
        messages: Vec::new(),
    };

    library.push(book1);
    library.push(book2);
    library.push(book3);

    list_books_in_library(&library);
    list_available_books_in_library(&library);
    show_first_n_books(&library, 2);

    find_book(&library, "The Rust Programming Language");
    find_book(&library, "The Python Programming Language");

    // ---- Demonstrate borrow/return + messages ----
    // Borrow the first book from the library
    if let Some(b) = library.get_mut(0) {
        user1.borrow_book(b);
    }

    // Try to borrow the same book again (should fail, message shouldn't duplicate)
    if let Some(b) = library.get_mut(0) {
        user1.borrow_book(b);
    }

    // Return the first book
    if let Some(b) = library.get_mut(0) {
        user1.return_book(b);
    }

    // Show user state
    user1.list_borrowed();
    user1.print_messages();

    // Show library availability again
    list_available_books_in_library(&library);

}
