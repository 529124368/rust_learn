use core::time;
use std::{
    ops::Deref,
    thread::{self, Thread},
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..100 {
            println!("线程运行中....{}", i);
            thread::sleep(time::Duration::from_millis(1))
        }
    });
    let f = |a: usize| -> usize {
        let mut sum = 0;
        sum = a + 1;
        sum
    };
    println!("{}", f(2));
    let ss = "sdfds".to_string();
    let a = |s| ss + s;
    println!("{}", a("sdfds"));
    let b = vec![1, 2, 3, 4, 5];
    let es: i32 = b.iter().sum();
    println!("{}", es);

    let x = Bbox::new(123);
    assert_eq!(123, *x);
    let aas = "safsdfs".to_string();
    assert_eq!(&aas, "safsdfs");
    handle.join().unwrap();
}

struct Bbox<T>(T);
impl<T> Bbox<T> {
    fn new(a: T) -> Self {
        Self(a)
    }
}
impl<T> Deref for Bbox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
