use crate::{
    util::{ItemOption, StatefulList},
    AvailableOption,
};

pub struct TerminalState<'a> {
    options_state: StatefulList<AvailableOption<'a>>,
    block_options: StatefulList<&'a str>,
    item_options: StatefulList<ItemOption<'a>>,
}
impl<'a> TerminalState<'a> {
    pub fn new(options: Vec<AvailableOption<'a>>, item_options: Vec<ItemOption<'a>>) -> Self {
        Self {
            options_state: StatefulList::of(options),
            item_options: StatefulList::of(item_options),
            block_options: StatefulList::of(vec![]),
        }
    }

    pub fn options(&mut self) -> &mut StatefulList<AvailableOption<'a>> {
        &mut self.options_state
    }

    pub fn block_options(&mut self) -> &mut StatefulList<&'a str> {
        &mut self.block_options
    }

    pub fn item_options(&mut self) -> &mut StatefulList<ItemOption<'a>> {
        &mut self.item_options
    }
}
