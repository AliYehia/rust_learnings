use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;


fn main() {
    let b_value = hashmap_lookup("x");
    match b_value {
        Some(value) => println!("Found the value {}", value), // can we assign the value returned to b_value?
        None => println!("Not found in the hashmap"),
    }

    let first_one = parse_from_string("123");
    match first_one {
        Ok(value) => println!("Parsed value: {}", value),
        Err(e) => println!("Error parsing string: {}", e),
    }
    let second_one = parse_from_string("abc");

    match second_one {
        Ok(value) => println!("Parsed value: {}", value),
        Err(e) => println!("Error parsing string: {}", e),
    }

    let result = read_number_from_file("number.txt");

    match result {
        Ok(value) => println!("Read and squared value: {}", value),
        Err(e) => println!("Error recieved from read_number_from_file: {}", e),
    }
}

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;          // may fail → io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;       // may fail → io::Error

    let num: i32 = contents.trim().parse()?;   // may fail → ParseIntError

    Ok(num * num)  // return squared number
}

fn hashmap_lookup(lookup_key: &str) -> Option<i32> {
    let mut map = std::collections::HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    return map.get(lookup_key).cloned();
}

fn parse_from_string(the_string: &str) -> Result<i32, ParseIntError> {
    the_string.parse::<i32>()
}
