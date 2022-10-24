use std::sync::mpsc;
use cursive::{
    event::Event,
    theme::{ Color, PaletteColor },
    view,
    views::{ EditView, SelectView },
    Cursive,
    CursiveRunnable,
    CursiveRunner
};

use crate::{controller::ControllerMessage, views::{AppView, self, ViewConfig}};
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
    SwitchView(views::ViewType)
}

impl Ui {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>, initial_view_type: views::ViewType) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();
        let mut ui = Ui {
            cursive: cursive::default().into_runner(),
            ui_tx: ui_tx,
            ui_rx: ui_rx,
            controller_tx: controller_tx,
        };

        ui.load_view(initial_view_type);
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
                },
                UiMessage::SwitchView(view_type) => {
                    self.switch(view_type);
                }
            }
        }

        if should_refresh {
            self.cursive.refresh();
        }

        true
    }

    fn load_view(&mut self, view_type: views::ViewType) {

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
                (PaletteColor::TitleSecondary, Color::TerminalDefault)
            ]);
        });

        self.build(view_type);

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

    fn build(&mut self, view_type: views::ViewType) {

        let view_config = ViewConfig {
            view_type: view_type,
            controller_tx: self.controller_tx.clone()
        };

        let current_view = views::ViewFactory::get(&view_config);
        if let Ok(that_view) = current_view.build() {
            self.cursive
                .add_layer(that_view);
        }
    }

    fn switch(&mut self, view_type: views::ViewType) {

        self.cursive.pop_layer();
        self.build(view_type);
        self.cursive.refresh();
    }
}
