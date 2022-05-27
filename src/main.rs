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
    block_with_text, bold, create_menu, italic, menu_spans, move_menu_spans, stop_editing_spans,
    AvailableOption, EditModeType, MenuType,
};
mod app;
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
                        KeyCode::Char('m') => app.set_mode(EditModeType::ItemOptions),
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
        EditModeType::None => {
            // First line
            let first_line = vec![
                Span::raw("Press "),
                Span::styled("e ", bold_style),
                Span::raw("to edit your namespace."),
            ];
            // Map to Spans which holds a vector of span
            vec![Spans::from(first_line), menu_spans()]
        }
        EditModeType::MainMenuOptions => {
            let line = vec![
                Span::raw("Press arrow "),
                Span::styled("up ", bold_style),
                Span::raw("or "),
                Span::styled("down ", bold_style),
                Span::raw("to select an option from the menu"),
            ];
            vec![Spans::from(line)]
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
    let text_widget = block_with_text(&app.mode, Paragraph::new(output), "Namespace");
    frame.render_widget(text_widget, area[1]);
    if app.mode == EditModeType::Namespace {
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

    let menu_widget = create_menu(
        "Select an option",
        items,
        app.mode == EditModeType::MainMenuOptions,
    );
    frame.render_stateful_widget(
        menu_widget,
        area[2],
        &mut app.state.options().current_state(),
    )
}

fn render_item_menu<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Percentage(80)].as_ref())
        .split(frame.size());

    // Create text lines
    let lines: Vec<Spans> = match app.mode {
        EditModeType::None => vec![menu_spans()],
        EditModeType::ItemOptions => vec![stop_editing_spans(), move_menu_spans()],
        _ => Vec::with_capacity(0),
    };
    frame.render_widget(Paragraph::new(lines), area[0]);
    let items: Vec<ListItem> = app
        .state
        .item_options()
        .elements()
        .iter()
        .map(|element| {
            // let square;
            // if element.is_active() {
            //     square = "[x]";
            // } else {
            //     square = format!("{} {}", "[ ]", element.get_desc()).as_str()
            // }
            let line = format!(
                "{} [{}] {}",
                element.get_option(),
                if element.is_active() { 'x' } else { ' ' },
                element.get_desc()
            );
            ListItem::new(Text::from(
                element.get_option().to_owned() + " " + line.as_str(),
            ))
        })
        .collect();

    let list = create_menu("Item Options", items, app.mode == EditModeType::ItemOptions);
    frame.render_stateful_widget(list, area[1], app.state.item_options().current_state());
}
