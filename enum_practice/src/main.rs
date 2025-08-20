fn main() {
    enum Grade {
        A,
        B,
        C,
        D,
        F,
    }

    enum Message {
        Text(String),
        Image(String),
        Video(String),
    }

    impl Message {
        fn deliver(&self) {
            match self {
                Message::Text(text) => println!("Delivering text: {}", text),
                Message::Image(file) => println!("Delivering image: {}", file),
                Message::Video(file) => println!("Delivering video: {}", file),
            }
        }
    }

    let grade1 = Grade::A;
    let grade2 = Grade::C;
    let grade3 = Grade::F;

    let msg1 = Message::Text(String::from("Words are meaningless"));
    let msg2 = Message::Image(String::from("photo.png"));
    let msg3 = Message::Video(String::from("withoutMe.mp4"));

    

    fn print_grade_message(grade: &Grade) {
        match grade {
            Grade::A => println!("Excellent!"),
            Grade::B => println!("Good job!"),
            Grade::C => println!("Average."),
            Grade::D => println!("Needs improvement."),
            _ => println!("Failed!"),
        }
    }

    fn print_message(msg: &Message) {
        match msg {
            Message::Text(text) => println!("Text message: {}", text),
            Message::Image(file) => println!("Image file: {}", file),
            Message::Video(file) => println!("Video file: {}", file),
        }
    }

    print_message(&msg1);
    print_message(&msg2);
    print_grade_message(&grade1);
    print_grade_message(&grade2);
    print_grade_message(&grade3);

    msg3.deliver();
}
