use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn create_menu<'a>(title: &'a str, entries: Vec<ListItem<'a>>, active: bool) -> List<'a> {
    let mut menu = List::new(entries).block(Block::default().borders(Borders::ALL).title(title));
    if active {
        menu = menu
            .highlight_style(
                Style::default()
                    .fg(Color::Rgb(255, 153, 0))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" > ");
    }
    return menu;
}

pub fn block_with_text<'a>(
    current_mode: &EditModeType,
    raw_paragraph: Paragraph<'a>,
    title: &'a str,
) -> Paragraph<'a> {
    raw_paragraph
        .style(match current_mode {
            EditModeType::Namespace => Style::default().fg(Color::Rgb(255, 153, 0)),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title(title))
}

pub fn bold() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}
pub fn italic() -> Style {
    Style::default().add_modifier(Modifier::ITALIC)
}
pub fn move_menu_spans<'a>() -> Spans<'a> {
    let bold = bold();
    let line = vec![
        Span::raw("Press arrow "),
        Span::styled("up ", bold),
        Span::raw("or "),
        Span::styled("down ", bold),
        Span::raw("to select an option from the menu."),
    ];
    Spans::from(line)
}
pub fn menu_spans<'a>() -> Spans<'a> {
    let line = vec![
        Span::raw("Press "),
        Span::styled("m ", bold()),
        Span::raw("to select an option from the menu."),
    ];
    Spans::from(line)
}
/// Reusable stop editing span vector
pub fn stop_editing_spans<'a>() -> Spans<'a> {
    let line = vec![
        Span::raw("Press "),
        Span::styled("Esc", bold()),
        Span::raw(" to stop editing."),
    ];
    Spans::from(line)
}
#[derive(PartialEq, Eq)]
pub enum EditModeType {
    Namespace,
    MainMenuOptions,
    ItemOptions,
    None,
}

#[derive(PartialEq, Eq)]
pub enum MenuType {
    MainMenu,
    ItemMenu,
    BlockMenu,
}
pub struct StatefulList<T> {
    state: ListState,
    list: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn of(list: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            list: list,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
    pub fn select_first(&mut self) {
        self.state.select(Some(0));
    }
    pub fn current_state(&mut self) -> &mut ListState {
        &mut self.state
    }
    pub fn elements(&self) -> &Vec<T> {
        &self.list
    }
    pub fn elements_mut(&mut self) -> &mut Vec<T> {
        &mut self.list
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
pub struct AvailableOption<'a> {
    option: &'a str,
    desc: &'a str,
}
impl<'a> AvailableOption<'a> {
    pub fn new(option: &'a str, desc: &'a str) -> AvailableOption<'a> {
        Self {
            option: option,
            desc: desc,
        }
    }
    pub fn get_option(&self) -> &'a str {
        self.option
    }
    pub fn get_desc(&self) -> &'a str {
        self.desc
    }
}

pub struct ItemOption<'a> {
    option: &'a str,
    desc: &'a str,
    active: bool,
}
impl<'a> ItemOption<'a> {
    pub fn new(option: &'a str, desc: &'a str) -> Self {
        Self {
            option,
            desc,
            active: false,
        }
    }
    pub fn get_option(&self) -> &'a str {
        self.option
    }
    pub fn get_desc(&self) -> &'a str {
        self.desc
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn toggle(&mut self) {
        self.active = !self.active
    }
}
