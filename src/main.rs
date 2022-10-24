extern crate env_logger;
#[macro_use] extern crate log;

use log::LevelFilter;
use std::error::Error;
use env_logger::Builder;

mod p2p;
mod ui;
mod state;
mod app;
mod controller;

fn main() -> Result<(), Box<dyn Error + 'static>> {

    // let mut log_builder = Builder::from_default_env();
    // log_builder
    //     .filter(None, LevelFilter::Info)
    //     .init();

    app::start();

    Ok(())
}