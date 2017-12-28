use std::sync::atomic::{AtomicPtr, Ordering, AtomicUsize};
use std::fmt::{Debug, Display, Formatter, Result};
use std::mem;

/*
 Generic list element
 */
 struct Node<T> {
     elem : T,
     next : Link<T>,
 }

impl<T> Debug for Node<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?} -> ", self.elem)
    }
}

#[derive(Debug)]
enum Link<T> {
    Empty,
    Cons(Box<Node<T>>),
}

struct Stack<T> {
    tail_ptr : Link<T>,
    ref_count : AtomicUsize,
}

impl<T> Stack<T> where T: Debug {
    fn new() -> Self {
        Stack {
            tail_ptr : Link::Empty,
            ref_count : std::sync::atomic::ATOMIC_USIZE_INIT,
        }
    }
    fn insert(&mut self, data : T) {
        /* Box of new Node */
        let new_node : Box<Node<T>> = Box::new ( Node {
            elem : data,
            next : mem::replace(&mut self.tail_ptr, Link::Empty),
        } );

        /* New tail Link */
        self.tail_ptr = Link::Cons(new_node);
    }

    fn debug(&self) {
        let mut itr = &self.tail_ptr;

        loop {
            match itr {
                &Link::Empty => break,
                &Link::Cons(ref x) => {
                    println!("{:?}", x);
                    itr = &x.next;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_one() {
        let mut test = Stack::<u32>::new();
        test.insert(10);
        test.debug();
    }

    #[test]
    fn insert_multi() {
        let mut test = Stack::<u32>::new();
        test.insert(10);
        test.insert(5);
        test.insert(20);
        test.debug();
    }
}
