use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

/* Handle the configs*/
#[derive(Debug)]
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
        let query = args[2].clone();
        let filename = args[1].clone();

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
    
    println!("{:?}", config);
    
    /* Pattern matching on return data */
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

use std::error::Error;
/* We need a function that returns unit Ok and can return many errors.
   One way could be :
        fn run<T> (config: Config) -> Result<(), T> where T : Error {
   But again here, calling run(config) will ask us to give a type T, which we can't know before hand.
   Error depends on runtime here. We should use Object traits.
 */
fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f. read_to_string(&mut contents)?;

    println!("With content:\n{:?}", contents);

   Ok(())
}
