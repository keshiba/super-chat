use std::sync::mpsc;

use crate::{ui::{Ui, UiMessage}, views};

pub struct Controller {
    rx: mpsc::Receiver<ControllerMessage>,
    ui: Ui,
}

pub enum ControllerMessage {
    UpdatedInputAvailable(String),
    SwitchView(views::ViewType)
}

impl Controller {
    pub fn new() -> Result<Controller, String> {
        let (tx, rx) = mpsc::channel::<ControllerMessage>();
        Ok(Controller {
            rx: rx,
            ui: Ui::new(tx.clone(), views::ViewType::LoginView),
        })
    }

    pub fn run(&mut self) {
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    ControllerMessage::UpdatedInputAvailable(text) => {
                        self.ui.ui_tx.send(UiMessage::UpdateOutput(text)).unwrap();
                    },
                    ControllerMessage::SwitchView(view_type) => {
                        self.ui.ui_tx.send(UiMessage::SwitchView(view_type)).unwrap();
                    },
                    _ => {}
                };
            }
        }
    }
}
