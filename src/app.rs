use catppuccin::PALETTE;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    prelude::*,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, List, ListItem, ListState,
        calendar::{self},
    },
};
use time::OffsetDateTime;

use crate::date_styler::{self, CompletedDateStyler};
use crate::my_colors;
use crate::user_habits;
use color_eyre::Result;
// use crate::my_colors;
// /// The main application which holds the state and logic of the application.
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
            show_add_habit: false,
            habit_stats: true,
            items: vec![
                user_habits::HabitItem {
                    name: String::from("Mediation"),
                    active: false,
                    id: 1,
                    frequency: 1,
                    current_streak: 1,
                    max_streak: 1,
                },
                // user_habits::HabitItem {
                //     name: String::from("Guitar"),
                // },
                // user_habits::HabitItem {
                //     name: String::from("Running"),
                // },
                // user_habits::HabitItem {
                //     name: String::from("Programming"),
                // },
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
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(outer_layout[0]);
        if self.habits.show_habit_list {
            // self.habit_list_block(outer_layout[0], frame.buffer_mut());
            let items = self.habits.items.clone();
            let (_items, list_widget) = Self::habit_list_block(&items);
            frame.render_stateful_widget(list_widget, outer_layout[0], &mut self.habits.state);
        }
        if self.habits.habit_calendar_track {
            // frame.render_widget(self.habits.habit_calendar_tracker_block(), inner_layout[0]);
            let habit_calendar_tracker_title_block = Block::new()
                .title(
                    Line::from("Habit Calendars Tracker")
                        .bold()
                        .blue()
                        .centered(),
                )
                .borders(Borders::ALL)
                .border_style(my_colors::BORDER_COL);

            let block = self.habit_calendar_tracker_block(&habit_calendar_tracker_title_block);
            if block.is_some() {
                frame.render_widget(&block, inner_layout[0]);
            } else {
                frame.render_widget(&habit_calendar_tracker_title_block, inner_layout[0]);
            }
        }
        if self.habits.habit_stats {
            frame.render_widget(self.habit_stats_tracker(), inner_layout[1])
        }
        if self.habits.show_add_habit {
            let text = Block::new()
                .title(Line::from("Add a habit here").bold().blue().centered())
                .borders(Borders::ALL)
                .border_style(my_colors::BORDER_COL);
            frame.render_widget(text, left_layout[1]);
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
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Char('h') | KeyCode::Left) => self.select_none(),
            (_, KeyCode::Char('j') | KeyCode::Down) => self.select_next(),
            (_, KeyCode::Char('k') | KeyCode::Up) => self.select_previous(),
            (_, KeyCode::Char('g') | KeyCode::Home) => self.select_first(),
            (_, KeyCode::Char('G') | KeyCode::End) => self.select_last(),
            (_, KeyCode::Esc) => self.habits.show_add_habit = false,
            (_, KeyCode::Char('a')) => self.habits.show_add_habit = true,
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
        if (!self.habits.show_add_habit) {
            self.running = false;
        }
    }

    pub fn habit_list_block(items: &'_ [user_habits::HabitItem]) -> (Vec<ListItem<'_>>, List<'_>) {
        let habit_list = Line::from("Habit List").bold().blue().centered();
        // .style(Style::new().fg(convert_color_type(PALETTE.macchiato.colors.blue)));

        let block = Block::new()
            .title(habit_list)
            .borders(Borders::ALL)
            .border_style(my_colors::BORDER_COL);

        let items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, list_item)| {
                let color = alternate_colors(i);
                ListItem::from(list_item.name.clone()).bg(color)
            })
            .collect();
        let list = List::new(items.clone())
            .block(block)
            .highlight_style(my_colors::SELECTED_STYLE)
            .highlight_symbol(">>")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        (items, list)
    }
    // this function needs a habit selected. So there must be data related to a habit
    pub fn habit_calendar_tracker_block<'a>(
        &self,
        habit_calendar_titile_block: &Block<'a>,
    ) -> Option<calendar::Monthly<'a, CompletedDateStyler>> {
        let date = OffsetDateTime::now_utc().date();
        let idx = self.habits.state.selected();
        if idx.is_some() {
            let date_styled_cal = CompletedDateStyler::new();
            let cal = calendar::Monthly::new(date, date_styled_cal)
                .block(habit_calendar_titile_block.clone())
                .show_month_header(Style::new().bold())
                .show_weekdays_header(Style::new().italic());
            return Some(cal);
        } else {
            return None;
        }
    }
    pub fn habit_stats_tracker(&self) -> Block<'_> {
        let habit_stats_title = Line::from("Habit Stats").bold().blue().centered();
        Block::default()
            .title(habit_stats_title)
            .borders(Borders::ALL)
            .border_style(my_colors::BORDER_COL)
    }
    // pub fn display_add_habit(&self)
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        my_colors::NORMAL_ROW_BG
    } else {
        my_colors::ALT_ROW_BG_COLOR
    }
}
