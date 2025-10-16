use crossterm::event::KeyCode;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    EnteringName,
    EnteringFrequency,
    MarkingDone,
    EnteringHours,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

impl InputMode {
    pub fn next(&mut self, code: KeyCode) {
        match (&self, code) {
            (InputMode::EnteringName, _) => *self = InputMode::EnteringFrequency,
            (InputMode::EnteringFrequency, _) => *self = InputMode::Normal,
            (InputMode::MarkingDone, _) => *self = InputMode::EnteringHours,
            (InputMode::EnteringHours, _) => *self = InputMode::Normal,
            (InputMode::Normal, KeyCode::Char('a')) => *self = InputMode::EnteringName,
            (InputMode::Normal, KeyCode::Tab) => *self = InputMode::MarkingDone,
            (_, _) => {}
        }
    }
    pub fn prev(&mut self) {
        match self {
            InputMode::EnteringFrequency => *self = InputMode::EnteringName,
            InputMode::EnteringHours => *self = InputMode::MarkingDone,
            _ => *self = InputMode::Normal,
        }
    }
}