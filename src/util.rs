use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub current: ListState,
    pub list: Vec<T>,
}

impl <T> StatefulList<T> {
    pub fn of(list: Vec<T>) -> Self {
        Self { 
            current: ListState::default(),
            list: list,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.current.selected()
    }
    pub fn next(&mut self) {
        let i = match self.current.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.current.selected() {
            Some(i) => {
                if i == 0 {
                    self.list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current.select(Some(i));
    }
} 