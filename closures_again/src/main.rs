use std::thread;
use std::time::Duration;
use std::hash::Hash;
use std::collections::HashMap;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        if let Some(&v) = self.value.get(&arg) {
            v
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number:u32) {
    let mut cached_result = Cacher::new(|num: u32| {
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            cached_result.value(intensity)
        );
        println!(
            "Next, do {} pushups",
            cached_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today");
        } else {
            println!(
                "Today, run for {} minutes",
                cached_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

// adding unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_same_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(1);
        assert_eq!(v1, 1);
        assert_eq!(v2, 1);
    }

    #[test]
    fn call_with_string() {
        let mut c = Cacher::new(|a: &str| a.len());
        let v1 = c.value("hello");
        let v2 = c.value("worldz");
        assert_eq!(v1, 5);
        assert_eq!(v2, 6);
    }
}