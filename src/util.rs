use tui::widgets::ListState;

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
    pub fn current_state(&mut self) -> &mut ListState {
        &mut self.state
    }
    pub fn elements(&self) -> &Vec<T> {
        &self.list
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
