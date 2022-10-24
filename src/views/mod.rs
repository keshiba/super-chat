use cursive::views::{LinearLayout, ResizedView};

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

pub struct ViewFactory;
impl ViewFactory {
    pub fn get(view_type: &ViewType) -> Box<dyn AppView> {

        match view_type {
            ViewType::LoginView => Box::new(login::LoginView::new()),
            ViewType::ChatWindowView => Box::new(chat::ChatWindowView::new()),
        }
    }
}