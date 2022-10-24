use std::sync::mpsc;

use crate::ui::{UiMessage, Ui};


pub struct Controller {
    rx: mpsc::Receiver<ControllerMessage>,
    ui: Ui
}

pub enum ControllerMessage {
    UpdatedInputAvailable(String)
}

impl Controller {
    pub fn new() -> Result<Controller, String> {
    
        let (tx, rx) = mpsc::channel::<ControllerMessage>();
        Ok(Controller { 
            rx: rx, 
            ui: Ui::new(tx.clone()) 
        })
    }

    pub fn run(&mut self) {
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    ControllerMessage::UpdatedInputAvailable(text) => {
                        self.ui
                            .ui_tx
                            .send(UiMessage::UpdateOutput(text))
                            .unwrap();
                    }
                };
            }
            self.ui.step();
        }
    }
}