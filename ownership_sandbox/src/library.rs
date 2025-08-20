pub fn run_library_demo() {
    let mut library: Vec<String> = Vec::new(); 

    let book1 = String::from("Waves");
    let book2 = String::from("Crabs");
    let book3 = String::from("Sunshine");

    // Add books 
    add_book(&mut library, String::from("The Hobbit")); 
    add_book(&mut library, String::from("1984")); 
    println!("Library now has {} books", library.len()); 
    
    // Borrow and check out first book 
    check_out_book(&library[0]); 
    // Duplicate second book 
    let copy = duplicate_book(&library[1]); 
    println!("Duplicate copy: {}", copy); 

    // Return a book after temporary ownership 
    let returned = return_book(library.pop().unwrap()); 
    println!("Returned book: {}", returned); 
    // Print library 
    println!("Library now has {} books", library.len()); 

    println!("--- Borrowing before move ---");
    check_out_book(&book1); // OK: immutable borrow

    check_out_book(&book1); // still valid since we only passed reference before

    println!("--- Moving ownership ---");
    add_book(&mut library, book1); // book1 is moved here
    // check_out_book(&book1); // ❌ Would fail: book1 was moved - we could alternativly use in line 31 book1.clone()

    add_book(&mut library, book2); // book2 moved

    println!("--- Cloning to preserve ownership ---");
    let book4 = duplicate_book(&book3); // book3 is still valid
    println!("Original book3: {}", book3);
    println!("Duplicate book4: {}", book4);

    println!("--- Taking ownership temporarily ---");
    let returned = return_book(library.pop().unwrap());
    println!("Returned book: {}", returned);

    println!("--- Library state ---");
    for b in &library {
        println!("Remaining book: {}", b);
    }
}

fn add_book(library: &mut Vec<String>, book: String) {
    println!("Book added: {}", book);
    library.push(book);
} 

fn check_out_book(book: &String) { 
    println!("Book checked out: {}", book); 
} 

fn duplicate_book(book: &String) -> String { 
    let duplicate = book.clone(); 
    
    duplicate 
}

fn return_book(book: String) -> String { 
    println!("Book returned: {}", book); 
    
    book 
}