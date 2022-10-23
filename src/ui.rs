use std::hint;

use cursive::View;
use cursive::direction::Direction;
use cursive::event;
use cursive::event::Event;
use cursive::theme;
use cursive::theme::BorderStyle;
use cursive::theme::Palette;
use cursive::theme::Theme;
use cursive::view;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::*;
use cursive::Cursive;
use std::sync::mpsc::{ Sender, Receiver };
use cursive::logger;

use crate::p2p::ChatError;
use crate::state;

pub const INPUTTEXTAREA_NAME: &str = "INPUT_TEXT_AREA";
pub const CHATWINDOWPANEL_NAME: &str = "CHAT_WINDOW_PANEL";
pub const CHATSELECTVIEW: &str = "CHAT_SELECT_VIEW";

pub fn start_ui(net_sender: &mut Sender<String>, ui_receiver: Receiver<String>) {

    logger::init();

    let mut net_sender_clone = net_sender.clone();
    let mut cursive_lib = cursive::default();
    let mut app_state = state::AppState::default();

    app_state.data.messages = vec![
        ("keshiba".to_owned(), "Hey people! What's up ?".to_owned()),
        ("anon1".to_owned(), "Hey, all good".to_owned()),
        ("keshiba".to_owned(), "Follow the white rabbit".to_owned()),
        ("anon1".to_owned(), "You high bro ?".to_owned()),
    ];

    cursive_lib.set_user_data(app_state);

    cursive_lib.set_theme({
        let mut theme = Theme::default();
        theme.shadow = false;
        theme
    });

    let mut chatwindow_list = SelectView::new();
    cursive_lib.with_user_data(|app_state: &mut state::AppState| {
        for message in &app_state.data.messages {
            chatwindow_list.add_item_str(format!{"{}: {}", message.0, message.1});
        }
    });

    let scroll_view = ScrollView::new(chatwindow_list.with_name(CHATSELECTVIEW))
        .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
        .show_scrollbars(true);

    let chatwindow_panel = ResizedView::with_full_screen(
            Panel::new(scroll_view))
            .with_name(CHATWINDOWPANEL_NAME);

    let input_textarea = EditView::new()
            .disabled()
            .on_submit(move |siv, text| {
                siv.call_on(&view::Selector::Name(INPUTTEXTAREA_NAME), |txt: &mut EditView| {
                    txt.set_content("");
                });
                net_sender_clone.send(text.to_string()).unwrap();
            })
            .with_name(INPUTTEXTAREA_NAME);
    let input_panel = Panel::new(input_textarea);
    let hint_panel = Panel::new(TextView::new("[i] - insert, [q] - quit"));

    let panels_list = LinearLayout::vertical()
        .child(chatwindow_panel.full_screen())
        .child(input_panel.fixed_height(3))
        .child(hint_panel.fixed_height(3));

    cursive_lib.add_layer(ResizedView::with_full_screen(panels_list));
    cursive_lib.add_global_callback('`', Cursive::toggle_debug_console);
    cursive_lib.add_global_callback('q', Cursive::quit);
    cursive_lib.add_global_callback('i', |a| {
        a.call_on_name(INPUTTEXTAREA_NAME, |txt: &mut EditView| {
            txt.enable();
        });
        a.focus_name(INPUTTEXTAREA_NAME);
    });

    cursive_lib.add_global_callback(Event::Key(cursive::event::Key::Esc), |a| {
        a.focus_name(CHATWINDOWPANEL_NAME);
        a.call_on(&view::Selector::Name(INPUTTEXTAREA_NAME), |edit_view: &mut EditView| {
            edit_view.disable();
        });
    });

    let mut runner = cursive_lib.runner();

    runner.refresh();

    loop {
        runner.step();

        if runner.is_running() == false {
            break;
        }

        let mut should_refresh = false;
        for m in ui_receiver.try_iter() {
            runner.call_on_name(CHATSELECTVIEW, |chat_window: &mut SelectView| {
                should_refresh = true;
                chat_window.add_item_str(m.to_string())
            });
        }

        if should_refresh {
            runner.refresh();
        }
    }
}
