use std::sync::mpsc;

use cursive::{views::{ResizedView, LinearLayout, Dialog, TextView, EditView}, view::SizeConstraint};

use crate::controller::ControllerMessage;

use super::{AppView, ViewConfig, chat::ChatWindowView};

pub struct LoginView {
    
    pub controller_tx: mpsc::Sender<ControllerMessage>
}

impl LoginView {

    pub fn new(view_config: &ViewConfig) -> Self {
        LoginView { 
            controller_tx: view_config.controller_tx.clone()
        }
    }
}

impl AppView for LoginView {

    fn build(&self) -> Result<ResizedView<LinearLayout>, String> {
    
        let controller_tx = self.controller_tx.clone();
    
        let dialog = Dialog::new()
            .title("Enter Nickname")
            .content(EditView::new().on_submit(move |siv, txt| {
                controller_tx
                    .clone()
                    .send(ControllerMessage::SwitchView(super::ViewType::ChatWindowView))
                    .unwrap();
            }));

        let layout = LinearLayout::vertical()
            .child(dialog);

        Ok(ResizedView::new(SizeConstraint::Free, SizeConstraint::Free, layout))
    }
}