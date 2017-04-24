use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

/* Handle the configs*/
#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensetive: bool,
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
        let case_sensetive = false;

        let case_sensetive = env::var("CASE_SENSETIVE").is_err();

        Ok(Config {
            query: query,
            filename: filename,
            case_sensetive: case_sensetive,
        })
    }

    pub fn query_string(&self) -> &str {
        &self.query
    }

    pub fn filename_string(&self) -> &str {
        &self.filename
    }

    pub fn is_case_sensetive(&self) -> bool {
        self.case_sensetive
    }
}

/* We need a function that returns unit Ok and can return many errors.
   One way could be :
        fn run<T> (config: Config) -> Result<(), T> where T : Error {
   But again here, calling run(config) will ask us to give a type T, which we can't know before hand.
   Error depends on runtime here. We should use Object traits.
 */
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename_string())?;
    let mut contents = String::new();
    f. read_to_string(&mut contents)?;

    match config.is_case_sensetive() {
        true => {
            for line in search(&config.query_string(), &contents) {
                println!("{}", line);
            }
        }
        false => {
            for line in search(&config.query_string().to_lowercase(), &contents.to_lowercase()) {
                println!("{:?}", line);
            }
        }

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
