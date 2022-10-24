extern crate env_logger;
#[macro_use] extern crate log;

use std::error::Error;

mod p2p;
mod ui;
mod state;
mod app;
mod controller;
mod views;

fn main() -> Result<(), Box<dyn Error + 'static>> {

    // let mut log_builder = Builder::from_default_env();
    // log_builder
    //     .filter(None, LevelFilter::Info)
    //     .init();

    app::start();

    Ok(())
}