#![allow(dead_code)]
struct CountUp {
current: usize,
}

#[derive(Debug)]
struct Fibonacci {
    curr: u32,
    next: u32,
}

/* The Iterator trait needs to implement the next method */
impl Iterator for CountUp {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        Some(self.current)
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self)-> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

struct Buffer<T> {
    size: usize,
    data: Vec<T>
}
/* the parameter type `T` may not live long enough.
 * reference type `&'a Buffer<T>` might outlive the data it points at
 * data: &'a Buffer<T>. Thus T should also have lifetime of 'a
 */
struct BufferIterator<'a, T> where T: 'a {
    data: &'a Buffer<T>,
    index: usize,
}


/* Implement a generic buffer */
impl<T> Buffer<T> {
    fn iter(&self) -> BufferIterator<T> {
        BufferIterator {
            data: self,
            index: 0,
        }
    }
}

/* Item has to be a reference or else the Item has to be a type
 * that implements Copy
 */
impl<'a, T> Iterator for BufferIterator<'a, T> where T:'a {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.size  {
                let ref _data = self.data.data[self.index];
                self.index = self.index + 1;
                Some(_data)
            } else {
                None
            }
    }
}

fn main() {
    let iterator = CountUp { current: 0 };
    let output = iterator.take(20).collect::<Vec<_>>();
    println!("{:?}", output);

    /* fib_it is moved when used with take. The iterators are consumed essentially. */
    /* We haven't built an iterable structure here, but just an iterator.*/
    /* Vec implement .iter() to create an iterator that take/collect can consume. That way, */
    /* Vec isn't affected. */
    let fib_it = Fibonacci { curr:1, next:2};
    let fib_out = fib_it.take(15).collect::<Vec<_>>();
    println!("{:?}", fib_out);

    let vec : Vec<String> = vec!["Hello".to_string(), "World".to_string(), "Good".to_string(), "Bye".to_string()];
    let buf = Buffer{size:4, data: vec};
    println!("{:?}", buf.iter().count());

    let vec : Vec<String> = vec!["Hello".to_string(), "World".to_string(), "Good".to_string(), "Bye".to_string()];
    /* 
     * Remember, if we do for x in vec, the values are moved and consumed by loop.
     * vec.iter() will implement iterators that will be consumed instead of data.
     * Rust seems to be move by default in most scenarios unless Copy is implemented.
     */
    for x in vec.iter() {
        println!("{:?}", x);
    }
    println!("{:?}", vec);
}
