use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();
    a.insert("adasd", 12);
    a.entry("ads");
    let mut p = Point::new();
    p.setX(123);
    p.setY(12);
    println!("{:#?}", p);
    println!("{}", p.getX());
    println!("{}", p.getY());
    let a = getob(&p);
    println!("{}", a);
    println!("{:?}", p);
    setob(&mut p, 11);
    println!("{:?}", p);
}
trait Flg: std::default::Default + Copy {}
impl<T> Flg for T where T: std::default::Default + Copy {}

fn getob<T: Flg>(p: &Point<T>) -> &T {
    p.getX()
}

fn setob<T: Flg>(p: &mut Point<T>, a: T) {
    p.setX(a);
}

#[derive(Default, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T: Flg> Point<T> {
    fn new() -> Self {
        Point::default()
    }

    fn getX(&self) -> &T {
        &self.x
    }

    fn getY(&self) -> &T {
        &self.y
    }
    fn setX(&mut self, x: T) {
        self.x = x
    }

    fn setY(&mut self, y: T) {
        self.y = y
    }
}
