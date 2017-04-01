use std::fmt::{Debug, Display, Formatter, Result};

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

/*
   impl Debug for List<String> can be made generic.
   We can define this impl Debug on T where T implements Display
 */
impl<T> Debug for List<T> where T : Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            List::Cons(ref value, ref ptr) => {
                /* print of ptr will call Debug again. We are doing pattern matching in recursion here */
                    print!("{:?}", ptr);
                /* When all children are printed, write current data */
                    write!(f,"{} -> ", value)
            }
            /* Base case of finding a List::Nil, just return Ok */
            List::Nil => Ok(()),
        }
    }
}

/*
   Using the where keyword we can add bound checks to what
   what this function works for.
   NOTE: x:&T takes the generic parameters as references
   */
fn printthis<T>(x:&T) where T : Display {
    println!("{}", x);
}

fn main() {
    let str_vec = vec!["Hello".to_string(), "World".to_string(), "Good".to_string(), "Bye".to_string()];
    let leaf = Box::new(List::Cons(4, Box::new(List::Nil)));
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, leaf)));
    let mut lea = Box::new(List::Nil);
    /* Create linked list out of vector */
    for string in str_vec {
        lea = Box::new(List::Cons(string, lea));
    }
    println!("{:?}", list);
    println!("{:?}", lea);
    let x : u32= 5;
    printthis::<u32>(&x);
}
