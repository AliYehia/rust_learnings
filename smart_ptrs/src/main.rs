use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

/// ref "&" points to a data - its the original pointer
/// no overhead, no special capabilities. They do not own the data.
/// Smart pointers are data structures that act like pointers but have more capabilities.
/// Smart pointers may own the data they point to in some cases. Examples: String & Vec<T>
/// Smart pointers implement Deref and Drop traits
/// 
/// Box<T> allows allocating values on the heap
/// Rc<T> a reference counting pointer that enables multiple ownership
/// Ref<T> amd RefMut<T> enables borrowing at runtime instead of compile time
/// 
/// When to use Box?
/// When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
/// When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
/// When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
/// 
/// When to use Rc?
/// When you want multiple owners to the same piece of data - and only use in single threaded programs


enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let mypox = MyBox::new(5);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
