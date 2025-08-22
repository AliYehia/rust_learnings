trait Describable {
    fn describe(&self) -> String;
}

struct Sword;
struct Shield;

impl Describable for Sword {
    fn describe(&self) -> String {
        "A sharp sword".to_string()
    }
}

impl Describable for Shield {
    fn describe(&self) -> String {
        "A sturdy shield".to_string()
    }
}

// STATIC DISPATCH: Generic function
fn print_description_static<T: Describable>(item: &T) {
    println!("Static: {}", item.describe());
}

// DYNAMIC DISPATCH: Trait object
fn print_description_dynamic(item: &dyn Describable) {
    println!("Dynamic: {}", item.describe());
}

fn main() {
    let sword = Sword;
    let shield = Shield;

    // STATIC DISPATCH: compiler knows exact types
    print_description_static(&sword);
    print_description_static(&shield);

    // DYNAMIC DISPATCH: trait objects
    let items: Vec<Box<dyn Describable>> = vec![Box::new(sword), Box::new(shield)];

    for item in items {
        print_description_dynamic(&*item); // deref Box to &dyn Describable
    }
}
