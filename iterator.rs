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


impl<T> IntoIterator for Buffer<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/*
 * into_iter for reference on buffer.
 * This can help us do for x in &buf
 * T here is buffer type and we define impl on &Buffer<T>
 * Item type is a reference to the elements &T
 * Iterator type is the BufferIterator we implemented.
 * into_iter needs to just return the BufferIterator using the iter()
 * implementation we did already.
 * This again highlights the point that iter() is same as into_iter(&self).
 */
impl<'a, T> IntoIterator for &'a Buffer<T> {
    type Item = &'a T;
    type IntoIter = BufferIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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

    /*
     * buf.iter() is just a shorthand for IntoIterator::into_iter(&buf).
     * The iter() impl used &self for accessing.
     */
    for x in buf.iter() {
        println!("{:?}", x);
    }
    /* Value not moved so we can still use. */
    println!("{:?}", buf.iter().count());

    /*
     * Once we implemeted the IntoIterator, we can use for x in buf directly. But into_iter takes value "into"
     * itself so a move happens, unless you implement an IntoIterator for reference and then do 
     * for x in &buf.
     */

    for x in &buf {
        println!("{:?}", x);
    }
    /* We used reference into_iter so we are good to use it again */

    for x in buf {
        println!("{:?}", x);
    }
    /* Value moved so we can't use anymore. */
    // println!("{:?}", buf.iter().count());

    let vec : Vec<String> = vec!["Trello".to_string(), "Troll".to_string(), "Food".to_string(), "Tye".to_string()];
    /* 
     * Remember, if we do for x in vec, the values are moved and consumed by loop.
     * vec.iter() will implement iterators that will be consumed instead of data.
     * Rust seems to be move by default in most scenarios unless Copy is implemented.
     */
    for x in vec.iter() {
        println!("{:?}", x);
    }
    println!("{:?}", vec);

    let buf = Buffer{size:4, data: vec};
    /* Value is moved using into_iter and collected in new vector v. */
    let v : Vec<String>  = buf.into_iter().collect();

    println!("{:?}", v);
}
