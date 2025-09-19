use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Borders},
};
#[derive(Debug, Default)]
pub struct UserHabits {
    pub show_habit_list: bool,
    pub habit_calendar_track: bool,
    pub habit_stats: bool,
}

impl UserHabits {
    pub fn habit_list_block(&self) -> Block<'_> {
        let habit_list = Line::from("Habit List").bold().blue().centered();
        return Block::default().title(habit_list).borders(Borders::ALL);
    }
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
