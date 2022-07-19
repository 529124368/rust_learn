use std::sync::atomic::{AtomicIsize, Ordering};

static state: AtomicIsize = AtomicIsize::new(0);
fn main() {
    state.store(12, Ordering::Relaxed);
    println!("{}", state.load(Ordering::Relaxed));
    let man = Man::new();
    println!("{}", man.age);
    man.say("2313".to_string(), 12313);
    //
}

#[derive(Default)]
struct Man<T, V>
where
    T: std::fmt::Display + std::default::Default,
    V: std::fmt::Display + std::default::Default,
{
    name: T,
    age: V,
}
impl<T, V> Man<T, V>
where
    T: std::fmt::Display + std::default::Default,
    V: std::fmt::Display + std::default::Default,
{
    fn say(&self, name: T, age: V) {
        println!("{},{}", name, age);
    }
    fn new() -> Self {
        Default::default()
    }
}
