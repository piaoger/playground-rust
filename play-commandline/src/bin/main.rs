
extern crate cloudstorage;
extern crate env_logger;
extern crate clap;

mod cli;
use cli::app_exec;

fn main() {
    env_logger::init().expect("Failed to initialize logger.");
    app_exec();
}

