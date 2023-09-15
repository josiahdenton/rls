use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    rls::run(args).unwrap_or_else(|err| {
        eprintln!("failed to run rls {}", err);
        process::exit(1);
    });
}
