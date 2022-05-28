use app::App;
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
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use util::{
    bold, create_menu, italic, menu_spans, move_menu_spans, stop_editing_spans, text_field,
    AvailableOption, EditMode, MenuType,
};
mod app;
mod state;
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
                // Global event handlers
                if KeyCode::Esc == key.code {
                    app.set_mode(EditMode::None);
                }
                if app.mode == EditMode::None && app.current_menu().can_navigate_back() {
                    if key.code == KeyCode::Char('q') {
                        app.navigate(app.menu.get_previous_menu());
                    }
                }
                // Screen-specific event handlers
                match app.current_menu() {
                    MenuType::MainMenu => {
                        // Only if we are on normal mode we can switch
                        match app.mode {
                            EditMode::None => {
                                match key.code {
                                    KeyCode::Char('e') => {
                                        app.set_mode(EditMode::Namespace);
                                    }
                                    KeyCode::Char('m') => {
                                        app.set_mode(EditMode::MainMenu);
                                        app.state.options().select_first()
                                    }
                                    _ => (),
                                };
                            }
                            EditMode::MainMenu => match key.code {
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
                            EditMode::Namespace => match key.code {
                                KeyCode::Char(c) => {
                                    app.namespace.push_str(&c.to_string().to_lowercase());
                                }
                                KeyCode::Backspace => {
                                    app.namespace.pop();
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    MenuType::ItemMenu => match app.mode {
                        EditMode::ItemMenu => match key.code {
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
                            KeyCode::Down => app.state.item_options().next(),
                            KeyCode::Up => app.state.item_options().previous(),
                            _ => (),
                        },
                        EditMode::ItemIdentifier => match key.code {
                            KeyCode::Char(c) => app
                                .state
                                .item_text_fields()
                                .get(0)
                                .data()
                                .extend(c.to_lowercase()),
                            KeyCode::Backspace => {
                                app.state.item_text_fields().get(0).data().pop();
                            }
                            _ => (),
                        },
                        EditMode::None => match key.code {
                            KeyCode::Char('e') => {
                                app.set_mode(EditMode::ItemIdentifier);
                            }

                            KeyCode::Char('m') => app.set_mode(EditMode::ItemMenu),
                            _ => (),
                        },
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
    let constrains = [
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Percentage(80),
    ];
    // Define area
    let area = Layout::default()
        .constraints(constrains.as_ref())
        .split(frame.size());
    // Bold style
    let bold_style = bold();
    // Create text lines
    let lines: Vec<Spans> = match &app.mode {
        EditMode::None => {
            // First line
            let first_line = vec![
                Span::raw("Press "),
                Span::styled("e ", bold_style),
                Span::raw("to edit your namespace."),
            ];
            // Map to Spans which holds a vector of span
            vec![Spans::from(first_line), menu_spans()]
        }
        EditMode::Namespace => {
            vec![stop_editing_spans()]
        }
        EditMode::MainMenu => {
            vec![move_menu_spans(), stop_editing_spans()]
        }
        _ => Vec::with_capacity(0),
    };
    // Convert Vect<Spans> to Text
    let text = Text::from(lines);
    // Create text widget
    let paragraph = Paragraph::new(text);
    // Render text
    frame.render_widget(paragraph, area[0]);
    // Create input text field
    let mut output = String::from(app.namespace.as_str());
    output.insert_str(0, " > ");
    let text_widget = text_field(
        &app.mode,
        EditMode::Namespace,
        Paragraph::new(output),
        "Namespace",
    );
    frame.render_widget(text_widget, area[1]);
    if app.mode == EditMode::Namespace {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        frame.set_cursor(
            // Put cursor past the end of the input text
            area[1].x + app.namespace.width() as u16 + 4, // symbol takes 3 spaces + 1 offset
            // Move one line down, from the border to the input line
            area[1].y + 1,
        )
    }
    // Render menu
    let items: Vec<ListItem> = app
        .state
        .options()
        .elements()
        .iter()
        .map(|entry| {
            let lines = vec![
                Spans::from(entry.get_option()),
                Spans::from(Span::styled(entry.get_desc(), italic())),
            ];
            return ListItem::new(lines).style(Style::default().fg(Color::White));
        })
        .collect();

    let menu_widget = create_menu("Select an option", items, app.mode == EditMode::MainMenu);
    frame.render_stateful_widget(
        menu_widget,
        area[2],
        &mut app.state.options().current_state(),
    )
}

fn render_item_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(frame.size());
    // Create text lines
    // Controllers render
    let lines: Vec<Spans> = match app.mode {
        EditMode::None => vec![
            Spans::from(vec![
                Span::raw("Press "),
                Span::styled("e ", bold()),
                Span::raw("to edit the item name."),
            ]),
            menu_spans(),
            Spans::from(vec![
                Span::raw("Press "),
                Span::styled("q ", bold()),
                Span::raw("to quit the current screen."),
            ]),
        ],
        EditMode::ItemMenu => vec![move_menu_spans(), stop_editing_spans()],
        EditMode::ItemIdentifier => vec![stop_editing_spans()],
        _ => Vec::with_capacity(0),
    };
    frame.render_widget(Paragraph::new(lines), area[0]);
    let items: Vec<ListItem> = app
        .state
        .item_options()
        .elements()
        .iter()
        .map(|element| {
            let line = format!(
                "[{}] {}:  {}",
                if element.is_active() { 'x' } else { ' ' },
                element.get_option(),
                element.get_desc()
            );
            ListItem::new(Text::from(line))
        })
        .collect();
    // TextFields render
    let text_fields = app.state.item_text_fields();
    let mut id = String::from(text_fields.get(0).data().as_str());
    id.insert_str(0, " > ");
    let text_field = text_field(
        &app.mode,
        EditMode::ItemIdentifier,
        Paragraph::new(id),
        text_fields.get(0).title(),
    );
    frame.render_widget(text_field, area[1]);

    if app.mode == EditMode::ItemIdentifier {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        frame.set_cursor(
            // Put cursor past the end of the input text
            area[1].x + text_fields.get(0).data().width() as u16 + 4, // symbol takes 3 spaces + 1 offset
            // Move one line down, from the border to the input line
            area[1].y + 1,
        )
    }
    if app.mode == EditMode::ItemDisplayName {}
    let list = create_menu("Item Options", items, app.mode == EditMode::ItemMenu);
    frame.render_stateful_widget(list, area[2], app.state.item_options().current_state());
}
// pub fn create_menu<'a>(title: &'a str, entries: Vec<ListItem<'a>>, active: bool) -> List<'a> {
//     let mut menu = List::new(entries).block(Block::default().borders(Borders::ALL).title(title));
//     if active {
//         menu = menu
//             .highlight_style(
//                 Style::default()
//                     .fg(Color::Rgb(255, 153, 0))
//                     .add_modifier(Modifier::BOLD),
//             )
//             .highlight_symbol(" > ");
//     }
//     return menu;
// }
