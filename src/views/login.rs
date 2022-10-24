use cursive::{views::{ResizedView, LinearLayout, Dialog, TextView, EditView}, view::SizeConstraint};

use super::AppView;

pub struct LoginView;

impl LoginView {

    pub fn new() -> Self {
        LoginView {  }
    }
}

impl AppView for LoginView {

    fn build(&self) -> Result<ResizedView<LinearLayout>, String> {
    
        let dialog = Dialog::new()
            .title("Enter Nickname")
            .content(EditView::new().on_submit(|siv, txt| {
                siv.quit()
            }));

        let layout = LinearLayout::vertical()
            .child(dialog);

        Ok(ResizedView::new(SizeConstraint::Free, SizeConstraint::Free, layout))
    }
}