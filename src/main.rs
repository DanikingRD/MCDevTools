use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute,
};
use util::StatefulList;
use std::{io, time::{Duration, Instant}};
use std::vec;
use tui::{
    backend::{CrosstermBackend, Backend}, 
    layout::{Layout, Constraint}, style::{Style, Color, Modifier},
    text::{Spans, Span}, widgets::{ListItem, List, Borders, Block}, Frame,Terminal,
};
mod util;
mod test;
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);    
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    run(&mut app, &mut terminal)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
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
        handle_events(app, timeout)?;
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}
fn handle_events(app: &mut App,timeout: Duration) -> Result<(), io::Error> {
    let event_available = crossterm::event::poll(timeout)?;
    if event_available {
        // If this is a keyboard event
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => app.state.next(),
                KeyCode::Up => app.state.previous(),
                KeyCode::Enter => {
                    let i = app.state.selected();
                    match i {
                        Some(val) => {
                            match val {
                                0 => println!("Generating item json...\r"),
                                1 => println!("Generating block json...\r"),
                                _ => (),
                            }
                        },
                        None => (),
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
fn render_app<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let area = Layout::default()
    .direction(tui::layout::Direction::Horizontal)
    .constraints([Constraint::Percentage(100)].as_ref())
    .split(frame.size());

    let items: Vec<ListItem> = app.state
    .list
        .iter()
        .map(|entry| {
            let mut lines = vec![
                Spans::from(entry.option)
            ];
            lines.push(Spans::from(Span::styled(
                entry.desc,
                Style::default().add_modifier(Modifier::ITALIC))));
                return ListItem::new(lines).style(Style::default().fg(Color::White));
            }).collect();
    let list = List::new(items)
    .block(Block::default().borders(Borders::ALL).title("Options"))  .highlight_style(
        Style::default()
            .fg(Color::Rgb(255, 153, 0))
            .add_modifier(Modifier::BOLD)
    ).highlight_symbol(" > ");
    frame.render_stateful_widget(list, area[0], &mut app.state.current);
}

/// This struct holds the current state of the app.
struct App<'a> {
    state: StatefulList<AvailableOption<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        
        Self {
            state: StatefulList::of(vec![
                AvailableOption::new("Create Item\n\n", "Generates JSON files for an item."),
                AvailableOption::new("Create Block", "Generates JSON files for a block."),
            ]),
        }
    }
    pub fn tick(&self) {

    }
}
struct AvailableOption<'a> {
    option: &'a str,
    desc: &'a str
}
impl <'a> AvailableOption<'a> {
    pub fn new(option: &'a str, desc: &'a str) -> AvailableOption<'a>{
        Self { option: option, desc: desc }
    } 
}