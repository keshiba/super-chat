use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, self}, style::ContentStyle,
};
use std::{io::{self, Stdout}, thread, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Block, Borders, ListItem, List},
    Terminal, Frame, layout::{Layout, Direction, Constraint, Alignment}, style::{Color, Style}, text::{Span, Spans},
};

enum InputMode {
    Normal,
    Editing
}

struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new()
        }
    }
}

pub fn start_app() -> Result<(), io::Error> {


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    run_app(app, &mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor();

    Ok(())
}

fn run_app<B: Backend>(mut app: App, terminal: &mut Terminal<B>) -> io::Result<()> {

    loop {
        terminal.draw(|frame| build_layout(&app, frame))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect())
                    },
                    KeyCode::Char(ch) => {
                        app.input.push(ch)
                    },
                    KeyCode::Backspace => {
                        app.input.pop();
                    },
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    },
                    _ => {}
                }
            }
        }
    };
}

fn build_layout<B: Backend>(app: &App, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(frame.size());

    let block = Block::default()
         .title(vec![
            Span::styled("Menu", Style::default().fg(Color::Gray))
         ])
         .title_alignment(Alignment::Right)
         .borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);

    let chat_window_block = Block::default()
         .title(vec![
            Span::styled("Chat", Style::default().fg(Color::Red)),
            Span::styled(" window", Style::default().fg(Color::Black))
         ])
         .title_alignment(Alignment::Right)
         .style(Style::default().bg(Color::White).fg(Color::Black))
         .borders(Borders::ALL)
         .border_style(Style::default().fg(Color::Black));

    let messages: Vec<ListItem> = 
        app.messages
            .iter()
            .enumerate()
            .map(|(index, msg)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", index, msg)))];
                ListItem::new(content)
            })
            .collect();
    
    let chat_window = List::new(messages)
            .block(chat_window_block);

    frame.render_widget(chat_window, chunks[1]);

    let block = Block::default()
         .title("Message")
         .title_alignment(Alignment::Right)
         .borders(Borders::ALL);
    frame.render_widget(block, chunks[2]);
}
