use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    /*
    for x in env::args() {
        println!(".. {:?}", x);
    }
    */
    let args: Vec<String> = env::args().collect();

    /*
       Cannot move out of indexed content.
       Will have to use ref or &.
       let filename = args[1];
     */
    let ref filename = args[1];

    let mut fd = File::open(filename).expect("File not found");
    let mut contents = String::new();
    fd.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("{:?}", contents);
}
