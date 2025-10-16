use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use time::{Date, OffsetDateTime};

use crate::app::App;
use crate::db::{self};
use crate::input_mode::InputMode;
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
            InputMode::EnteringHours => self.habit_hours_buffer.handle_key(key.code),
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
                self.input_mode.next(key.code);
                self.habits.show_add_habit = true;
            }
            (_, KeyCode::Char('d')) => {
                let idx = self.get_current_habit();
                if idx.is_some() {
                    self.db
                        .delete_habit(idx.unwrap())
                        .expect("this habit id does not exist");
                }
            }
            (_, KeyCode::Tab) => {
                self.input_mode.next(key.code);
            }
            _ => {}
        }
    }
    fn handle_input_done(&mut self, key: KeyEvent) {
        self.habit_hours_buffer.handle_key(key.code);
        match key.code {
            KeyCode::BackTab => {
                self.input_mode.prev();
            }
            KeyCode::Left => {
                //display previous months calendar
                todo!()
            }
            KeyCode::Right => {
                todo!()
            }
            KeyCode::Enter => {
                let idx = self.habits.state.selected();
                if idx.is_some() {
                    self.db.add_completed(
                        &OffsetDateTime::now_utc().date(),
                        &self.habits.items[idx.unwrap()],
                        self.habit_hours_buffer.content.parse().unwrap_or(0),
                    );
                }
            }
            // maybe this mode is view only for now. Mark done should just happen on the left bar
            _ => {}
        }
    }
    fn handle_name_input(&mut self, key: KeyEvent) {
        self.habit_name_buffer.handle_key(key.code);
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.input_mode.next(key.code);
            }
            KeyCode::BackTab | KeyCode::Esc => {
                self.input_mode.prev();
                self.habits.show_add_habit = false;
            }
            _ => {}
        }
    }

    fn handle_freq_input(&mut self, key: KeyEvent) {
        self.habit_freq_buffer.handle_key(key.code);
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                self.input_mode.next(key.code);
                self.habits.show_add_habit = false;
                let new_habit = self.db.add_habit(
                    &self.habit_name_buffer.content,
                    &self
                        .habit_freq_buffer
                        .content
                        .parse()
                        .expect("Failed to parse frequency"),
                );
                if new_habit.is_ok() {
                    self.habits.items.push(new_habit.unwrap());
                }
            }
            KeyCode::BackTab => {
                self.input_mode.prev();
            }
            KeyCode::Esc => {
                self.input_mode.prev();
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
