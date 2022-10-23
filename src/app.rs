use std::thread;
use std::sync::mpsc;

pub fn start() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let (mut ui_sender, ui_receiver) = mpsc::channel::<String>();
    let (mut net_sender, net_receiver) = mpsc::channel::<String>();

    let mut ui_sender_clone = ui_sender.clone();

    let _handle = thread::spawn(move || {

        crate::p2p::start(&mut ui_sender, net_receiver);
    });

    ui_sender_clone.send(String::from("Wabba labba dubb dubb"));

    // crate::ui::start_ui(&mut net_sender, ui_receiver);
    crate::ui::start_ui(&mut ui_sender_clone, ui_receiver);

    Ok(())
}