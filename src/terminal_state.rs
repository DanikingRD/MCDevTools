use crate::{util::StatefulList, AvailableOption};

pub struct TerminalState<'a> {
    options_state: StatefulList<AvailableOption<'a>>,
    block_options: StatefulList<&'a str>,
    item_options: StatefulList<&'a str>,
}
impl<'a> TerminalState<'a> {
    pub fn new(available_options: Vec<AvailableOption<'a>>) -> Self {
        Self {
            options_state: StatefulList::of(available_options),
            block_options: StatefulList::of(vec!["example"]),
            item_options: StatefulList::of(vec!["example"]),
        }
    }
    pub fn options(&mut self) -> &mut StatefulList<AvailableOption<'a>> {
        &mut self.options_state
    }

    pub fn block_options(&mut self) -> &mut StatefulList<&'a str> {
        &mut self.block_options
    }

    pub fn item_options(&mut self) -> &mut StatefulList<&'a str> {
        &mut self.item_options
    }
}
