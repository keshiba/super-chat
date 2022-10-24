use std::thread;
use std::sync::mpsc;

use crate::controller::Controller;

pub fn start() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let (mut ui_sender, ui_receiver) = mpsc::channel::<String>();
    let (mut net_sender, net_receiver) = mpsc::channel::<String>();

    let mut ui_sender_clone = ui_sender.clone();

    let _handle = thread::spawn(move || {

        crate::p2p::start(&mut ui_sender, net_receiver);
    });

    match Controller::new() {

        Ok(mut controller) => controller.run(),
        Err(e) => println!("Error: {}", e)
    }
    // crate::ui::start_ui(&mut ui_sender_clone, ui_receiver);

    Ok(())
}