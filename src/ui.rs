use cursive::event::Event;
use cursive::theme::Color;
use cursive::theme::PaletteColor;
use cursive::view;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::EditView;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::ResizedView;
use cursive::views::ScrollView;
use cursive::views::SelectView;
use cursive::views::TextView;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::CursiveRunner;
use std::sync::mpsc;

use crate::controller::ControllerMessage;

use crate::state;

pub const INPUTTEXTAREA_NAME: &str = "INPUT_TEXT_AREA";
pub const CHATWINDOWPANEL_NAME: &str = "CHAT_WINDOW_PANEL";
pub const CHATSELECTVIEW: &str = "CHAT_SELECT_VIEW";

pub struct Ui {
    cursive: CursiveRunner<CursiveRunnable>,
    ui_rx: mpsc::Receiver<UiMessage>,
    controller_tx: mpsc::Sender<ControllerMessage>,
    pub ui_tx: mpsc::Sender<UiMessage>,
}

pub enum UiMessage {
    UpdateOutput(String),
}

impl Ui {
    pub fn new<'a>(controller_tx: mpsc::Sender<ControllerMessage>) -> Ui {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();
        let mut ui = Ui {
            cursive: cursive::default().into_runner(),
            ui_tx: ui_tx,
            ui_rx: ui_rx,
            controller_tx: controller_tx,
        };

        ui.build();
        ui.cursive.refresh();

        ui
    }

    pub fn step(&mut self) -> bool {
        self.cursive.step();

        if self.cursive.is_running() == false {
            return false;
        }

        let mut should_refresh = false;
        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UiMessage::UpdateOutput(update_message) => {
                    if let Some(mut select_view) =
                        self.cursive.find_name::<SelectView>(CHATSELECTVIEW)
                    {
                        should_refresh = true;
                        select_view.add_item_str(update_message);
                    }
                }
            }
        }

        if should_refresh {
            self.cursive.refresh();
        }

        true
    }

    fn build(&mut self) {
        let net_sender_clone = self.controller_tx.clone();
        let mut app_state = state::AppState::default();

        app_state.data.messages = vec![
            ("keshiba".to_owned(), "Hey people! What's up ?".to_owned()),
            ("anon1".to_owned(), "Hey, all good".to_owned()),
            ("keshiba".to_owned(), "Follow the white rabbit".to_owned()),
            ("anon1".to_owned(), "You high bro ?".to_owned()),
        ];

        self.cursive.set_user_data(app_state);

        self.cursive.update_theme(|theme| {
            theme.shadow = false;
            theme.palette.extend(vec![
                (PaletteColor::Background, Color::TerminalDefault),
                (PaletteColor::View, Color::TerminalDefault),
                (PaletteColor::Primary, Color::TerminalDefault),
                (PaletteColor::TitlePrimary, Color::TerminalDefault),
                (PaletteColor::TitleSecondary, Color::TerminalDefault),
            ]);
        });

        let mut chatwindow_list = SelectView::new();
        self.cursive
            .with_user_data(|app_state: &mut state::AppState| {
                for message in &app_state.data.messages {
                    chatwindow_list.add_item_str(format! {"{}: {}", message.0, message.1});
                }
            });

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
                net_sender_clone
                    .send(ControllerMessage::UpdatedInputAvailable(text.to_string()))
                    .unwrap();
            })
            .with_name(INPUTTEXTAREA_NAME);
        let input_panel = Panel::new(input_textarea);
        let hint_panel = Panel::new(TextView::new("[i] - insert, [q] - quit"));

        let panels_list = LinearLayout::vertical()
            .child(chatwindow_panel.full_screen())
            .child(input_panel.fixed_height(3))
            .child(hint_panel.fixed_height(3));

        self.cursive
            .add_layer(ResizedView::with_full_screen(panels_list));
        self.cursive
            .add_global_callback('`', Cursive::toggle_debug_console);
        self.cursive.add_global_callback('q', Cursive::quit);
        self.cursive.add_global_callback('i', |a| {
            a.call_on_name(INPUTTEXTAREA_NAME, |txt: &mut EditView| {
                txt.enable();
            });
            a.focus_name(INPUTTEXTAREA_NAME);
        });

        self.cursive
            .add_global_callback(Event::Key(cursive::event::Key::Esc), |a| {
                a.focus_name(CHATWINDOWPANEL_NAME);
                a.call_on(
                    &view::Selector::Name(INPUTTEXTAREA_NAME),
                    |edit_view: &mut EditView| {
                        edit_view.disable();
                    },
                );
            });
    }
}
