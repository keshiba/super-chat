use cursive::{views::{ResizedView, LinearLayout, Panel, ScrollView, SelectView, EditView, TextView}, view::{self, ViewWrapper, Resizable, Nameable}};

use crate::controller::ControllerMessage;

use super::AppView;

pub const INPUTTEXTAREA_NAME: &str = "INPUT_TEXT_AREA";
pub const CHATWINDOWPANEL_NAME: &str = "CHAT_WINDOW_PANEL";
pub const CHATSELECTVIEW: &str = "CHAT_SELECT_VIEW";

pub struct ChatWindowView;


impl ChatWindowView {
    pub fn new() -> Self {
        ChatWindowView {}
    }
}

impl AppView for ChatWindowView {

    fn build(&self) -> Result<ResizedView<LinearLayout>, String> {

        let mut chatwindow_list = SelectView::<String>::new();
        // self.cursive
        //     .with_user_data(|app_state: &mut state::AppState| {
        //         for message in &app_state.data.messages {
        //             chatwindow_list.add_item_str(format! {"{}: {}", message.0, message.1});
        //         }
        //     });

        let scroll_view = ScrollView::new(chatwindow_list.with_name(CHATSELECTVIEW))
            .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
            .show_scrollbars(true);

        let chatwindow_panel =
            ResizedView::with_full_screen(Panel::new(scroll_view)).with_name(CHATWINDOWPANEL_NAME);

        let input_textarea = EditView::new()
            .disabled()
            .on_submit(move |siv, text| {
                siv.call_on(
                    &view::Selector::Name(INPUTTEXTAREA_NAME),
                    |txt: &mut EditView| {
                        txt.set_content("");
                    },
                );
                // net_sender_clone
                //     .send(ControllerMessage::UpdatedInputAvailable(text.to_string()))
                //     .unwrap();
            })
            .with_name(INPUTTEXTAREA_NAME);
        let input_panel = Panel::new(input_textarea);
        let hint_panel = Panel::new(TextView::new("[i] - insert, [q] - quit"));

        let panels_list = LinearLayout::vertical()
            .child(chatwindow_panel.full_screen())
            .child(input_panel.fixed_height(3))
            .child(hint_panel.fixed_height(3));

        Ok(ResizedView::with_full_screen(panels_list))
    }
}