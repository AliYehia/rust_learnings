use std::collections::HashMap;

fn main() {
    // create the hashmap <String, Vec<i32>>
    let mut students_grades: HashMap<String, Vec<i32>> = HashMap::new();

    add_grade(&mut students_grades, "Ashraf", 90);
    add_grade(&mut students_grades, "Ashraf", 60);
    add_grade(&mut students_grades, "Som3a", 90);
    add_grade(&mut students_grades, "Som3a", 97);
    add_grade(&mut students_grades, "Som3a", 92);
    add_grade(&mut students_grades, "Shiko", 85);
    add_grade(&mut students_grades, "Shiko", 87);
    add_grade(&mut students_grades, "Shiko", 92);

    print_all_students_grades(&students_grades);

    print_student_grades(&students_grades, "Ashraf");
    print_student_grades(&students_grades, "Somza");

    print_student_grades_average(&students_grades, "Som3a");

    cleanup_hashmap(&mut students_grades);
    remove_student(&mut students_grades, "Shiko");
    print_all_students_grades(&students_grades);

    // create a function that adds to the hashmap
    fn add_grade(students_grades: &mut HashMap<String, Vec<i32>>, student: &str, grade: i32) {
        // .entry will check the hashmap if the key (student) exists, will return the value or none
        // .or_insert is what will happen if the key does not exist, it will create Vec::new()
        // .push either way this grade to that vec
        students_grades.entry(student.to_string()).or_insert(Vec::new()).push(grade);
    }

    // create a function that will search the hashmap and print out the grades
    fn print_all_students_grades(students_grades: &HashMap<String, Vec<i32>>) {
        for (student, scores) in students_grades {
            println!("Student: {}, scored grades: {:?}", student, scores);
        }
    }

    fn print_student_grades(students_grades: &HashMap<String, Vec<i32>>, student: &str) {
        if let Some(grades) = students_grades.get(student) {
            println!("Student: {}, scored grades: {:?}", student, grades);
        } else {
            println!("No grades found for student: {}", student);
        }
    }

    fn print_student_grades_average(students_grades: &HashMap<String, Vec<i32>>, student: &str) {
        if let Some(grades) = students_grades.get(student) {
            let grades_total:usize = grades.len();
            let grades_sum:i32 = grades.iter().sum();
            let avg = grades_sum as f32 / grades_total as f32;
            println!("Student {} does got an average {}", student, avg)
        } else {
            println!("Student {} does not exist in system", student)
        }
    }

    fn cleanup_hashmap(students_grade: &mut HashMap<String, Vec<i32>>) {
        println!("Cleaning Up!!");
        students_grade.retain(|_student, grades| grades.len() > 2);
    }

    fn remove_student(students_grades: &mut HashMap<String, Vec<i32>>, student: &str) {
        if let Some(student1) = students_grades.get(student) {
            println!("Removing student: {:?} from the system", student);
            students_grades.remove(student);
        } else {
            println!("Student : {:?} not in the system", student);
        }
        /*
        chatGs suggestion
        match students_grades.remove(student) {
            Some(_) => println!("Removed student {} from the system", student),
            None => println!("Student {} not in the system", student),
        }
     */
        
    }
}
