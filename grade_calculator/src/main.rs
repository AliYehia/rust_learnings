fn main() {
    println!("Hello, world!");

    let grades = [20, 78, 92, 64, 10];

    for grade in grades {
        let score = calculate_grade(grade);
        println!("You scored an {} because your grade was {}!", score, grade);
    }

    fn calculate_grade(grade: i32) -> char {
        if grade >= 90 {
            'A'
        } else if grade >= 80 {
            'B'
        } else if grade >= 70 {
            'C'
        } else if grade >= 60 {
            'D'
        } else {
            'F'
        }
    }
}
