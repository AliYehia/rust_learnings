fn main() {
    let mut nums: Vec<i32> = vec![1,2,3,4,5];

    // print each number using a for loop
    println!("====Looping over the vec====");
    nums.iter().for_each(|x: &i32| println!("We have a number: {}", x) );

    // double each number in place
    nums.iter_mut().for_each(|x: &mut i32| *x *= 2 );

    println!("\n====Looping after doubling the vec====");
    nums.iter().for_each(|x: &i32| println!("We have a number: {}", x) );

    let fruits = vec!["apple", "banana", "cherry"];
    fruits.into_iter().for_each(|fruit| println!("Fruit: {}", fruit) );

    let nums2 = vec![1,2,3,4,5];
    let squared: Vec<i32> = nums2.into_iter().map(|x| x * x ).collect();
    println!("\n====Squared numbers====");
    squared.iter().for_each(|x| println!("Squared number: {}", x) );

    let nums3 = vec![10,15,20,25,30];
    // filter numbers that are not divisible by 10
    // why did we need to use &x here? 

    let filtered: Vec<i32> = nums3.iter().filter(|&x| x 
        % 10 == 0).cloned().collect();
    println!("\n====Filtered numbers (divisible by 10)====");
    filtered.iter().for_each(|x| println!("Filtered number: {}", x) );
    
    
    println!("\n====Only the first 2 numbers in filtered====");
    let first_three: Vec<i32> = filtered.iter().take(2).cloned().collect();
    first_three.iter().for_each(|x| println!("First three number: {}", x));

    let num4 = vec![1,2,3,4,5];
    let greater_than_2 = num4.iter().find(|&&x| x > 4);
    match greater_than_2 {
        Some(value) => println!("Found a number greater than 2 : {}", value),
        None => println!("No number greater than 2 found"),
    }

    let mut final_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10,20];
    // filter numbers that are divisible by 10, square them, and take the first 3
    let final_challange: Vec<i32> = final_nums.iter().filter(|&x| x % 10 == 0).map(|&y| y * y).take(3).collect();
    println!("The vector is: {:?}", final_challange);

    let mut words = vec![
        String::from("this"),
        String::from("is"),
        String::from("words"),
        String::from("mean"),
        String::from("nothingatall"),
    ];

    words.iter().for_each(|w| println!("word: {}", w));

    let short_words: Vec<_> = words
        .iter()
        .filter(|w| w.len() <= 3)
        .collect();

    println!("short words: {:?}", short_words);

    words.iter_mut().for_each(|w| w.push_str("!")); 
    println!("after exclamation: {:?}", words);

    
}
