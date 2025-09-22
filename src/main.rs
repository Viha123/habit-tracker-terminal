use color_eyre::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    style::{
        Color, Modifier, Style, Stylize,
        palette::tailwind::{BLUE, GREEN, SLATE},
    },
    widgets::ListState,
    widgets::{Block, Borders, List, ListItem, StatefulWidget},
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

use crate::user_habits::HabitItem;
// use std::f64::consts::PI;
mod user_habits;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    habits: user_habits::UserHabits,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        self.habits = user_habits::UserHabits {
            show_habit_list: true,
            habit_calendar_track: true,
            habit_stats: true,
            items: vec![
                user_habits::HabitItem {
                    name: String::from("Mediation"),
                },
                user_habits::HabitItem {
                    name: String::from("Guitar"),
                },
                user_habits::HabitItem {
                    name: String::from("Running"),
                },
                user_habits::HabitItem {
                    name: String::from("Programming"),
                },
            ],
            state: ListState::default(),
        };
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        // let x: u16 = ((frame.count() as f64).sin() * 10.0 + 20.0).floor() as u16;
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(frame.area());
        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(outer_layout[1]);
        if self.habits.show_habit_list {
            // self.habit_list_block(outer_layout[0], frame.buffer_mut());
            let items = self.habits.items.clone();
            let list_widget= self.habit_list_block(items);
            frame.render_stateful_widget(
                list_widget,
                outer_layout[0],
                &mut self.habits.state,
            );
        }
        if self.habits.habit_calendar_track {
            // frame.render_widget(self.habits.habit_calendar_tracker_block(), inner_layout[0]);
            frame.render_widget(self.habit_calendar_tracker_block(), inner_layout[0]);
        }
        if self.habits.habit_stats {
            frame.render_widget(self.habit_stats_tracker(), inner_layout[1])
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
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
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Char('h') | KeyCode::Left) => self.select_none(),
            (_, KeyCode::Char('j') | KeyCode::Down) => self.select_next(),
            (_, KeyCode::Char('k') | KeyCode::Up) => self.select_previous(),
            (_, KeyCode::Char('g') | KeyCode::Home) => self.select_first(),
            (_, KeyCode::Char('G') | KeyCode::End) => self.select_last(),
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
        self.running = false;
    }

    pub fn habit_list_block(&self, items: Vec<user_habits::HabitItem>) -> List<'_> {
        let habit_list = Line::from("Habit List").bold().blue().centered();
        let block = Block::new()
            .title(habit_list)
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);
        let items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, list_item)| {
                let color = alternate_colors(i);
                ListItem::from(list_item.name.clone()).bg(color)
            })
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        return list;
    }
    // this function needs a habit selected. So there must be data related to a habit
    pub fn habit_calendar_tracker_block(&self) -> Block<'_> {
        let habit_calendar_tracker_title = Line::from("Habit Calendars Tracker")
            .bold()
            .blue()
            .centered();
        return Block::default()
            .title(habit_calendar_tracker_title)
            .borders(Borders::ALL);
    }
    pub fn habit_stats_tracker(&self) -> Block<'_> {
        let habit_stats_title = Line::from("Habit Stats").bold().blue().centered();
        Block::default()
            .title(habit_stats_title)
            .borders(Borders::ALL)
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
