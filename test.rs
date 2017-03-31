use std::fmt::{Debug, Formatter, Result};

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl Debug for List<std::string::String> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            List::Cons(ref ptr, ref X) => {
                    write!(f,"{} ->", ptr)
                    
            }
            List::Nil => Ok(()),
        }
    }
}

fn main() {
    let str_vec = vec!["Hello".to_string(), "World".to_string(), "Good".to_string(), "Bye".to_string()];
    let leaf = Box::new(List::Cons(4, Box::new(List::Nil)));
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, leaf)));
    let mut lea = Box::new(List::Nil);
    for string in str_vec {
        lea = Box::new(List::Cons(string, lea));
    }
    //println!("{:?}", list);
    println!("{:?}", lea);
}

