use std::{process, env};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    rls::run().unwrap_or_else(|| {
        eprintln!("failed to run rls");
        process::exit(1);
    });
}
