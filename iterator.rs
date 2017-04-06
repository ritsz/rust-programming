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
}
