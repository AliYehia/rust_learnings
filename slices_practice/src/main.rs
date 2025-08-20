fn main() {
    let s1 = String::from("Rust is fine");
    let v1:Vec<i32> = Vec::from([1,2,3,4,5,6]);

    show_str_slice(&s1, 0, 5);
    add_vec_slice(&v1, 2, 5);

    println!("word slice is: {}", &s1[0..5])
}

fn show_str_slice(word: &String, start: usize, end: usize) {
    println!("word slice is: {}", &word[start..end])
}

fn add_vec_slice(arr: &Vec<i32>, start:usize, end:usize) {
    // let sum: i32 = arr[start..end].iter().sum(); - also works
    let slice: &[i32] = &arr[start..end];
    let sum: i32 = slice.iter().sum();
    println!("Sum of slice is: {}", sum);
}