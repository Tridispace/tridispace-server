extern crate tridispace_server;

fn main() {
    // Run our server
    tridispace_server::run_server().expect("Server crashed");
}

