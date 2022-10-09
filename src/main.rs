extern crate env_logger;
#[macro_use] extern crate log;

use async_std::{task};
use log::LevelFilter;
use std::error::Error;
use env_logger::Builder;

use superchat::*;

fn main() {
    task::block_on(async_main());
}

async fn async_main() -> Result<(), Box<dyn Error>> {

    let mut log_builder = Builder::from_default_env();
    log_builder
        .filter(None, LevelFilter::Info)
        .init();

    init().await
}