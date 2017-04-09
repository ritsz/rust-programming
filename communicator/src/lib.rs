extern crate pnet;

use pnet::datalink::{self, NetworkInterface};

/* Define a module here. Implementation will look for a file named server.rs */
mod server;

mod layer;

/* Different modules can have same names. */
mod client {
    fn connect() {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
