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

    /*
       When writing generic code, it is often desirable to abstract over 
       all ways of borrowing data from a given type. That is the role of 
       the Borrow trait: if T: Borrow<U>, then &U can be borrowed from &T.
       A given type can be borrowed as multiple different types. In particular,
       Vec<T>: Borrow<Vec<T>> and Vec<T>: Borrow<[T]>.
     */
    foo(&i);
    foo(&mut i);
}
