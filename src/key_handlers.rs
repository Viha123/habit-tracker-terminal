use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{App, InputMode};

impl App {
    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    pub fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match (key.modifiers, key.code) {
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                // Add other key handlers here.
                (_, KeyCode::Char('h') | KeyCode::Left) => self.select_none(),
                (_, KeyCode::Char('j') | KeyCode::Down) => self.select_next(),
                (_, KeyCode::Char('k') | KeyCode::Up) => self.select_previous(),
                (_, KeyCode::Char('g') | KeyCode::Home) => self.select_first(),
                (_, KeyCode::Char('G') | KeyCode::End) => self.select_last(),
                (_, KeyCode::Esc) => self.habits.show_add_habit = false,
                (_, KeyCode::Char('a')) => {
                    self.input_mode = InputMode::EnteringName;
                    self.habit_name_buffer.clear();
                    self.habits.show_add_habit = true;
                }
                _ => {}
            },
            InputMode::EnteringName => match key.code {
                KeyCode::Enter | KeyCode::Tab => {
                    self.input_mode = InputMode::EnteringFrequency;
                    self.habit_freq_buffer.clear();
                }
                KeyCode::BackTab => {
                    self.input_mode = InputMode::Normal;
                    self.habit_name_buffer.clear();
                    self.habits.show_add_habit = false;
                }
                KeyCode::Char(c) => self.habit_name_buffer.push(c),
                KeyCode::Backspace => {
                    self.habit_name_buffer.pop();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.habits.show_add_habit = false;
                }
                _ => {}
            },
            InputMode::EnteringFrequency => match key.code {
                KeyCode::Enter | KeyCode::Tab => {
                    self.input_mode = InputMode::Normal;
                    self.habits.show_add_habit = false;
                }
                KeyCode::BackTab => {
                    self.input_mode = InputMode::EnteringName;
                    self.habit_freq_buffer.clear();
                }
                KeyCode::Char(c) => self.habit_freq_buffer.push(c),
                KeyCode::Backspace => {
                    self.habit_freq_buffer.pop();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.habits.show_add_habit = false;
                }
                _ => {}
            },
        }
    }
    fn select_none(&mut self) {
        self.habits.state.select(None);
    }
    fn select_next(&mut self) {
        self.habits.state.select_next();
    }
    fn select_previous(&mut self) {
        self.habits.state.select_previous();
    }
    fn select_first(&mut self) {
        self.habits.state.select_first();
    }
    fn select_last(&mut self) {
        self.habits.state.select_last();
    }
    /// Set running to false to quit the application.
    fn quit(&mut self) {
        if !self.habits.show_add_habit {
            self.running = false;
        }
    }
}
