use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    io,
    time::{Duration, Instant},
};
use std::{io::Stdout, vec};
use terminal_state::TerminalState;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Widget},
    Frame, Terminal,
};
use util::{AvailableOption, MenuType, StatefulList};
mod terminal_state;
mod test;
mod util;
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    render(&mut app, &mut terminal)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

// Render method, this is the main loop that renders all the TUI
fn render(
    app: &mut App,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), io::Error> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| match app.menu {
            MenuType::MainMenu => render_options_menu(app, frame),
            MenuType::BlockMenu => render_block_menu(app, frame),
            _ => (),
        })?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        let event_available = crossterm::event::poll(timeout)?;
        if event_available {
            // If this is a keyboard event
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        if *app.current_menu() == MenuType::MainMenu {
                            app.state.options().next();
                        }
                    }
                    KeyCode::Up => {
                        if *app.current_menu() == MenuType::MainMenu {
                            app.state.options().previous();
                        }
                    }
                    KeyCode::Enter => {
                        let i = app.state.options().selected();
                        match i {
                            Some(val) => match val {
                                0 => {
                                    app.navigate(MenuType::BlockMenu);
                                }
                                1 => println!("Generating block json...\r"),
                                _ => (),
                            },
                            None => (),
                        }
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}

fn render_block_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());
    let items = vec![
        ListItem::new(Text::from("[]")),
        ListItem::new(Text::from("[]")),
    ];
    let list = List::new(items);
    frame.render_widget(list, area[0]);
}

fn render_options_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let items: Vec<ListItem> = app
        .state
        .options()
        .elements()
        .iter()
        .map(|entry| {
            let mut lines = vec![Spans::from(entry.get_option())];
            lines.push(Spans::from(Span::styled(
                entry.get_desc(),
                Style::default().add_modifier(Modifier::ITALIC),
            )));
            return ListItem::new(lines).style(Style::default().fg(Color::White));
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Options"))
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 153, 0))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" > ");
    frame.render_stateful_widget(list, area[0], &mut app.state.options().current_state())
}

/// This struct holds the current state of the app.
struct App<'a> {
    state: TerminalState<'a>,
    menu: MenuType,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            state: TerminalState::new(vec![
                AvailableOption::new("Create Item\n\n", "Generates JSON files for an item."),
                AvailableOption::new("Create Block", "Generates JSON files for a block."),
            ]),
            menu: MenuType::MainMenu,
        }
    }
    pub fn navigate(&mut self, menu: MenuType) {
        self.menu = menu
    }
    pub fn current_menu(&self) -> &MenuType {
        &self.menu
    }
    pub fn tick(&self) {}
}
