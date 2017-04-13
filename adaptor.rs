use std::fmt;

enum Many {
    Number(i32),
    Alpha(String),
}

impl fmt::Debug for Many {
    fn fmt(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Many::Number(num) => write!(fm, "{}", num),
            Many::Alpha(ref string) => write!(fm, "{}", string),
        }
    }
}

/* Takes a string and parses it to Many */
use std::str::FromStr;
impl FromStr for Many {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i32::from_str(s) {
            Err(_)  => Ok(Many::Alpha(s.to_string())),
            Ok(_)   => Ok(Many::Number(i32::from_str(s).unwrap())),
        }
    }
}

fn main() {
    /* Lets use an adaptor sequence to create a vector of Many */
    /*
        A string can have string characters or numbers split with .
    */
    /* Unwrap is used to get the value from inside Some() */
    let vec = "123.HELLO.345.WORLD".split('.').map(|s| s.parse::<Many>().unwrap()).collect::<Vec<_>>();
    println!("{:?}", vec);
}
