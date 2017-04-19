use std::borrow::Borrow;
use std::fmt::Display;

/* 
   foo runs for anything that borrows i32
   So it will work for both & and &mut
 */
fn foo<T: Borrow<i32> + Display>(a: T) {
        println!("a is borrowed: {}", a);
}


fn main() {
    let mut i = 5;

    foo(&i);
    foo(&mut i);
}
