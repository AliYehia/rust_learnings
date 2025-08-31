/// Implementing a calculator that does only one thing
/// takes a number, and adds 1 to it
/// But it's a very expensive process! So we are caching the result of each number in a hashmap
/// If we do have the value in the hashmap return it
/// Else execute the closure!

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct LazyCalculator <T,U,V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    function: T,
    results: HashMap<U,V>,
}

impl <T,U,V> LazyCalculator<T,U,V> 
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new (function: T) -> Self {
        LazyCalculator { 
            function, 
            results: HashMap::new(), 
        }
    }

    fn calculate (&mut self, args: U) -> V {
        if let Some(&v) = self.results.get(&args) {
            v
        } else {
            let v = (self.function)(args);
            self.results.insert(args, v);
            v
        }
    }
}

fn main() {
    let my_closure = |numz: u32| {
        println!("executing a very long process of adding 1 to the number {}", numz);
        thread::sleep(Duration::from_secs(2));
        numz + 1
    };

    let mut myLC = LazyCalculator::new(my_closure);

    println!("Lets do 1+1: {}", myLC.calculate(1));
    println!("Lets do 1+1 again?: {}", myLC.calculate(1));
    println!("Lets do 2+1: {}", myLC.calculate(2));
}
