trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn do_something(x: &Foo) {
    println!("{:?}", x.method());
}

fn main() {
    let x : u8 = 5;
    do_something(&x as &Foo);
}
