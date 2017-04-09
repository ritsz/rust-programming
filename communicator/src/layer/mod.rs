use super::server;

pub mod network;

fn datalink() {
}

/* Uses super to access connect of parent */
fn use_server() {
    server::connect();
}
