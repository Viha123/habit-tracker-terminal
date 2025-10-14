use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use time::{Date, OffsetDateTime};

use crate::app::{App, InputMode};
use crate::db::{self};
use crate::user_habits::HabitItem;
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
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::EnteringName => self.handle_name_input(key),
            InputMode::EnteringFrequency => self.handle_freq_input(key),
            InputMode::MarkingDone => self.handle_input_done(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
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
            (_, KeyCode::Tab) => {
                self.input_mode = InputMode::MarkingDone;
            }
            _ => {}
        }
    }
    fn handle_input_done(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::BackTab => {
                self.input_mode = InputMode::Normal;
            }
            
            KeyCode::Enter => {
                let idx = self.habits.state.selected();
                if idx.is_some() {
                    self.db.add_completed(
                        &OffsetDateTime::now_utc().date(),
                        &self.habits.items[idx.unwrap()],
                        1,
                    );
                }
            }
            // maybe this mode is view only for now. Mark done should just happen on the left bar
            _ => {}
        }
    }
    fn handle_name_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.input_mode = InputMode::EnteringFrequency;
                self.habit_freq_buffer.clear();
            }
            KeyCode::BackTab | KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.habit_name_buffer.clear();
                self.habits.show_add_habit = false;
            }
            KeyCode::Char(c) => self.habit_name_buffer.push(c),
            KeyCode::Backspace => {
                self.habit_name_buffer.pop();
            }
            _ => {}
        }
    }

    fn handle_freq_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.input_mode = InputMode::Normal;
                self.habits.show_add_habit = false;
                let new_habit = self.db.add_habit(
                    &self.habit_name_buffer,
                    &self
                        .habit_freq_buffer
                        .parse()
                        .expect("Failed to parse frequency"),
                );
                if new_habit.is_ok() {
                    self.habits.items.push(new_habit.unwrap());
                }
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
