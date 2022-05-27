use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, self, Event},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use std::{io, time::{Duration, Instant}};
use std::vec;
use tui::{
    backend::{CrosstermBackend, Backend}, 
    layout::{Layout, Constraint}, style::{Style, Color, Modifier},
    text::Spans, widgets::{ListItem, List, Borders, Block, ListState}, Frame,Terminal,
};
mod test;
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    run(&mut app, &mut terminal)?;
    disable_raw_mode()?;

    Ok(())
    
}
fn run<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame|render_app(app, frame))?;
        let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0));
        handle_events(timeout)?;
    }
}
fn handle_events(timeout: Duration) -> Result<(), io::Error> {
    let event_available = crossterm::event::poll(timeout)?;
    if event_available {
        // If this is a keyboard event
        if let Event::Key(key) = event::read()? {
            println!("Keyboard event");
        }
    }
    Ok(())
}
fn render_app<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
    .direction(tui::layout::Direction::Horizontal)
    .constraints([Constraint::Percentage(100)].as_ref())
    .split(frame.size());

    let items: Vec<ListItem> = app.options
        .iter()
        .map(|entry| {
            let lines = vec![Spans::from(entry.as_str())];
            ListItem::new(lines).style(Style::default())
        }).collect();
    let list = List::new(items)
    .block(Block::default().borders(Borders::ALL).title("List"))  .highlight_style(
        Style::default()
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
    ).highlight_symbol(">> ");
    frame.render_stateful_widget(list, area[0], &mut app.state);
}

struct App {
    options: Vec<String>,
    state: ListState,
}
impl App {
    pub fn new() -> Self {
        Self {
            options: vec![
                "Item1".to_string(),
                "Item2".to_string(),
                "Item3".to_string(),
            ],
            state: ListState::default(),
        }
    }
}
