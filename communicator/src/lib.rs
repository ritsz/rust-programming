extern crate pnet;

use pnet::datalink::{self, NetworkInterface};

/* Define a module here. Implementation will look for a file named server.rs */
pub mod server;

/* Different module, looks for a folder named layer and a file layer/mod.rs */
pub mod layer;

/* Different modules can have same names. */
/* Make the module public */
pub mod client {
    /* Made the function publically available */
    pub fn connect() -> Result<u32, u32> {
        Err(1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

    #[test]
    fn explore() {
        use client;
        assert_eq!(client::connect(), Err(1));
    }
}
