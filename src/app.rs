use crate::{
    terminal_state::TerminalState,
    util::{AvailableOption, EditModeType, ItemOption, MenuType},
};

/// This struct holds the current state of the app.
pub struct App<'a> {
    pub namespace: String,
    pub mode: EditModeType,
    pub state: TerminalState<'a>,
    pub menu: MenuType,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            namespace: String::from("modid"),
            mode: EditModeType::None,
            state: TerminalState::new(
                vec![
                    AvailableOption::new("Create Item", "Generates JSON files for an item."),
                    AvailableOption::new("Create Block", "Generates JSON files for a block."),
                ],
                vec![ItemOption::new("Handheld")],
            ),
            menu: MenuType::MainMenu,
        }
    }
    pub fn navigate(&mut self, menu: MenuType) {
        self.mode = EditModeType::None;
        self.menu = menu;
    }
    pub fn current_menu(&self) -> &MenuType {
        &self.menu
    }
    pub fn set_mode(&mut self, mode: EditModeType) {
        self.mode = mode;
    }
    pub fn tick(&self) {}
}