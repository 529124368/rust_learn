use std::any::type_name;

fn main() {
    let asd = vec![12, 12, 23, 343, 4343];
    let add = [12, 12, 23, 343, 4343];
    // let add = &add;
    // let asd = &asd;

    assert_eq!(&asd, &add);

    println!("{}", type_of(asd));
    println!("{}", type_of(add));
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
