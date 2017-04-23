use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

/* Handle the configs*/
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

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

/* We need a function that returns unit Ok and can return many errors.
   One way could be :
        fn run<T> (config: Config) -> Result<(), T> where T : Error {
   But again here, calling run(config) will ask us to give a type T, which we can't know before hand.
   Error depends on runtime here. We should use Object traits.
 */
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f. read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

/* This search can be a private function */
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
