use std::sync::mpsc;

use cursive::views::{LinearLayout, ResizedView};

use crate::controller::ControllerMessage;

use self::login::LoginView;

pub mod chat;
pub mod login;

pub trait AppView {
    fn build(&self) -> Result<ResizedView<LinearLayout>, String>;
}

pub enum ViewType {
    LoginView,
    ChatWindowView
}

pub struct ViewConfig {
    pub view_type: ViewType,
    pub controller_tx: mpsc::Sender<ControllerMessage>
}

pub struct ViewFactory;
impl ViewFactory {
    pub fn get(view_config: &ViewConfig) -> Box<dyn AppView> {

        match view_config.view_type {
            ViewType::LoginView => Box::new(login::LoginView::new(&view_config)),
            ViewType::ChatWindowView => Box::new(chat::ChatWindowView::new(&view_config)),
        }
    }
}