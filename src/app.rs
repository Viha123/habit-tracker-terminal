use crossterm::event::{KeyCode, KeyboardEnhancementFlags};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    prelude::*,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph,
        calendar::{self},
    },
};
use time::OffsetDateTime;

use crate::text_input;
use crate::user_habits;
use crate::{date_styler::CompletedDateStyler, my_colors::SELECTED_STYLE};
use crate::{db::db, text_input::TextInput};
use crate::{input_mode::InputMode, my_colors};
use color_eyre::Result;
// /// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub habits: user_habits::UserHabits,
    pub input_mode: InputMode,
    pub habit_freq_buffer: TextInput,
    pub habit_hours_buffer: TextInput,
    pub habit_hours_done: bool,
    pub habit_name_buffer: TextInput,
    pub db: db,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        self.input_mode = InputMode::Normal;
        self.habits = user_habits::UserHabits {
            show_habit_list: true,
            habit_calendar_track: true,
            show_add_habit: false,
            habit_stats: true,
            items: vec![],
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
        let enter_hours_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(inner_layout[0]);
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(outer_layout[0]);
        if self.habits.show_habit_list {
            // self.habit_list_block(outer_layout[0], frame.buffer_mut());
            self.habits.items = self.db.get_habits().clone();
            let items = self.habits.items.clone();
            let (_items, list_widget) = Self::habit_list_block(&items);
            frame.render_stateful_widget(list_widget, outer_layout[0], &mut self.habits.state);
        }
        if self.habits.habit_calendar_track {
            // frame.render_widget(self.habits.habit_calendar_tracker_block(), inner_layout[0]);
            let mut border_style = my_colors::NORMAL_STYLE;
            if self.input_mode == InputMode::MarkingDone {
                border_style = SELECTED_STYLE;
            }
            let habit_calendar_tracker_title_block = Block::new()
                .title(
                    Line::from("Habit Calendars Tracker")
                        .bold()
                        .blue()
                        .centered(),
                )
                .borders(Borders::ALL)
                .border_style(border_style);

            let block = self.habit_calendar_tracker_block(&habit_calendar_tracker_title_block);
            if block.is_some() {
                frame.render_widget(&block, enter_hours_layout[0]);
            } else {
                frame.render_widget(&habit_calendar_tracker_title_block, enter_hours_layout[0]);
            }
            // add a text input here
            let hours_block = Block::new()
                .title(
                    Line::from("Log Hours")
                        .bold()
                        .blue()
                        .centered(),
                )
                .borders(Borders::ALL)
                .border_style(border_style);
            let hours_paragraph = Paragraph::new(self.habit_hours_buffer.content.clone())
                .centered()
                .block(hours_block);
            frame.render_widget(hours_paragraph, enter_hours_layout[1]);
        }
        if self.habits.habit_stats {
            frame.render_widget(self.habit_stats_tracker(), inner_layout[1])
        }
        if self.habits.show_add_habit {
            self.display_add_habit(frame, left_layout[1]);
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
        let idx = self.get_current_habit();

        if idx.is_some() {
            let temp_vec = self.db.list_completed_dates(idx.unwrap());
            let mut date_styled_cal = CompletedDateStyler::new();
            date_styled_cal
                .update_dates(temp_vec)
                .expect("updated message");
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
    pub fn display_add_habit(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        // Split the area vertically for title, name input, and frequency input
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                // /// The main application which holds the state and logic of the application.
                Constraint::Length(3), // Title block
                Constraint::Length(3), // Name input
                Constraint::Length(3), // Frequency input
            ])
            .split(area);

        // Title block
        let title_block = Block::new()
            .title(Line::from("Add a habit here").bold().blue().centered())
            .borders(Borders::ALL)
            .border_style(my_colors::BORDER_COL);
        frame.render_widget(title_block, chunks[0]);

        // Habit name input block
        let mut name_style: Style = my_colors::NORMAL_STYLE;
        if self.input_mode == InputMode::EnteringName {
            name_style = SELECTED_STYLE;
        }
        let name_block = Block::new()
            .title("Habit Name")
            .borders(Borders::ALL)
            .border_style(name_style);
        // Habit name input block
        let mut para_style: Style = my_colors::NORMAL_STYLE;
        if self.input_mode == InputMode::EnteringFrequency {
            para_style = SELECTED_STYLE;
        }

        let name_paragraph = Paragraph::new(self.habit_name_buffer.content.clone())
            .centered()
            .block(name_block);
        frame.render_widget(name_paragraph, chunks[1]);

        // Habit frequency input block
        let freq_block = Block::new()
            .title("Frequency (Times/Week)")
            .borders(Borders::ALL)
            .border_style(para_style);
        let freq_paragraph = Paragraph::new(self.habit_freq_buffer.content.clone())
            .centered()
            .block(freq_block);
        frame.render_widget(freq_paragraph, chunks[2]);
    }

    pub fn get_current_habit(&self) -> Option<u64> {
        let idx = self.habits.state.selected();
        if idx.is_some() {
            return Some(self.habits.items[idx.unwrap()].id);
        } else {
            return None;
        }
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        my_colors::NORMAL_ROW_BG
    } else {
        my_colors::ALT_ROW_BG_COLOR
    }
}
