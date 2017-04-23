use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
/* Handle the configs*/
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

       /* Instead of panic we can send reasonable Result messages */
        if args.len() < 3
        {
            return Err("not enough Args");
        }

        /* Create a clone of the values so that they can be used as values in Config */
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}


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

    /* Unwrap of handle the error using closure */
    let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
            });
    
    let mut fd = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    fd.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("{:?}", contents);
}
