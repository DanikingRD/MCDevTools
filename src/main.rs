use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use std::{io::Stdout, vec};
use terminal_state::TerminalState;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use util::{AvailableOption, EditModeType, ItemOption, MenuType};
mod terminal_state;
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
            MenuType::ItemMenu => render_item_menu(app, frame),
            MenuType::BlockMenu => render_block_menu(app, frame),
        })?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        let event_available = crossterm::event::poll(timeout)?;
        if event_available {
            // If this is a keyboard event
            if let Event::Key(key) = event::read()? {
                // Update modes
                if KeyCode::Esc == key.code {
                    app.set_mode(EditModeType::None);
                }
                match app.current_menu() {
                    MenuType::MainMenu => {
                        // Only if we are on normal mode we can switch
                        match app.mode {
                            EditModeType::None => {
                                match key.code {
                                    KeyCode::Char('e') => {
                                        app.set_mode(EditModeType::Namespace);
                                    }
                                    KeyCode::Char('m') => {
                                        app.set_mode(EditModeType::MainMenuOptions);
                                        app.state.options().select_first()
                                    }
                                    _ => (),
                                };
                            }
                            EditModeType::MainMenuOptions => match key.code {
                                KeyCode::Down => app.state.options().next(),
                                KeyCode::Up => app.state.options().previous(),
                                KeyCode::Enter => {
                                    let index = app.state.options().selected();
                                    match index {
                                        Some(val) => match val {
                                            0 => {
                                                app.navigate(MenuType::ItemMenu);
                                                app.state.item_options().select_first()
                                            }
                                            1 => {
                                                app.navigate(MenuType::BlockMenu);
                                                app.state.block_options().select_first();
                                            }
                                            _ => (),
                                        },
                                        None => (),
                                    }
                                }
                                _ => (),
                            },
                            EditModeType::Namespace => match key.code {
                                KeyCode::Char(c) => app.namespace.push(c),
                                KeyCode::Backspace => {
                                    app.namespace.pop();
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    MenuType::ItemMenu => match key.code {
                        KeyCode::Char(' ') => {
                            let options = app.state.item_options();
                            let index = options.selected();
                            match index {
                                Some(pos) => match options.elements_mut().get_mut(pos) {
                                    Some(item) => item.toggle(),
                                    None => (),
                                },
                                None => (),
                            }
                        }
                        _ => (),
                    },
                    _ => (),
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
    frame.render_stateful_widget(list, area[0], app.state.block_options().current_state());
}

fn render_options_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    // Define constrains for widgets
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(frame.size());
    // Controls desc
    let input_msg = match app.mode {
        EditModeType::None => vec![
            Span::raw("Press "),
            Span::styled("e ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("to edit your namespace, "),
            Span::styled("m ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("to select an option from the menu"),
        ],
        _ => vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to stop editing."),
        ],
    };
    // Create helper message
    let input_text = Text::from(Spans::from(input_msg));
    let help_message = Paragraph::new(input_text);
    // Render message
    frame.render_widget(help_message, area[1]);
    if app.mode == EditModeType::MainMenuOptions {
        // Append another message for main menu
        let msg = vec![
            Span::raw("Press arrow "),
            Span::styled("up ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("or "),
            Span::styled("down ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("to select an option from the menu"),
        ];
        let paragraph = Paragraph::new(Text::from(Spans::from(msg)));
        frame.render_widget(paragraph, area[0]);
    }
    // Create input text field
    let mut output = String::from(app.namespace.as_str());
    output.insert_str(0, " > ");
    let input = Paragraph::new(output)
        .style(match app.mode {
            EditModeType::Namespace => Style::default().fg(Color::Rgb(255, 153, 0)),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Namespace"));
    frame.render_widget(input, area[2]);
    match app.mode {
        EditModeType::Namespace => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                area[2].x + app.namespace.width() as u16 + 4, // symbol takes 3 spaces + 1 offset
                // Move one line down, from the border to the input line
                area[2].y + 1,
            )
        }
        _ => (),
    };
    // Render menu
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

    let mut list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select an option"),
    );
    if app.mode == EditModeType::MainMenuOptions {
        list = list
            .highlight_style(
                Style::default()
                    .fg(Color::Rgb(255, 153, 0))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" > ");
    }

    frame.render_stateful_widget(list, area[3], &mut app.state.options().current_state())
}

fn render_item_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let items: Vec<ListItem> = app
        .state
        .item_options()
        .elements()
        .iter()
        .map(|element| {
            let square;
            if element.is_active() {
                square = "[x]";
            } else {
                square = "[]";
            }
            ListItem::new(Text::from(element.get_option().to_owned() + " " + square))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Item Options"))
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 153, 0))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" > ");
    frame.render_stateful_widget(list, area[0], app.state.item_options().current_state());
}

/// This struct holds the current state of the app.
struct App<'a> {
    namespace: String,
    mode: EditModeType,
    state: TerminalState<'a>,
    menu: MenuType,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            namespace: String::from("modid"),
            mode: EditModeType::None,
            state: TerminalState::new(
                vec![
                    AvailableOption::new("Create Item\n\n", "Generates JSON files for an item."),
                    AvailableOption::new("Create Block", "Generates JSON files for a block."),
                ],
                vec![ItemOption::new("Handheld")],
            ),
            menu: MenuType::MainMenu,
        }
    }
    pub fn navigate(&mut self, menu: MenuType) {
        self.menu = menu;
    }
    pub fn current_menu(&self) -> &MenuType {
        &self.menu
    }
    pub fn set_mode(&mut self, mode: EditModeType) {
        self.mode = mode;
    }
    pub fn tick(&self) {}
}
