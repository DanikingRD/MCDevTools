use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::io;
use std::vec;
use tui::{
    backend::CrosstermBackend, layout::Layout, style::Style, text::Spans, widgets::ListItem, Frame,
    Terminal,
};
mod test;
fn main() -> Result<(), io::Error> {
    enable_raw_mode();
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::new();
    loop {
        terminal.draw(|frame| {
            println!("Drawing()");
        })?;
    }
}
fn draw_app(app: &mut App) {
    // let layout = Layout::default();
    // app.options
    //     .iter()
    //     .map(|entry| {
    //         let mut lines = vec![Spans::from(entry.as_str())];
    //         ListItem::new(lines).style(Style::default());
    //     })

    //     .collect();
}

struct App {
    options: Vec<String>,
}
impl App {
    pub fn new() -> Self {
        Self {
            options: vec![
                "Item1".to_string(),
                "Item2".to_string(),
                "Item3".to_string(),
            ],
        }
    }
}
