/* Tuple struct. fields don't have a name */
#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    /* black can be used with .0, .1 etc to get the members */
    let black_g = black.1;
    println!("{:?}", black_g);
}
