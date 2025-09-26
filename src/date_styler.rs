use crate::my_colors;
use ratatui::{style::Style, widgets::calendar};
use time::Date;
// hold completed dates from database
pub struct CompletedDateStyler {
    completed_dates: Vec<Date>,
}

impl calendar::DateStyler for CompletedDateStyler {
    fn get_style(&self, date: Date) -> ratatui::prelude::Style {
        for d in &self.completed_dates {
            if *d == date {
                return my_colors::SELECTED_STYLE;
            }
        }
        return Style::new();
    }
}

impl CompletedDateStyler {
    pub fn new() -> Self {
        // test dates
        Self {
            completed_dates: vec![
                Date::from_calendar_date(2025, time::Month::September, 20).unwrap(),
                Date::from_calendar_date(2025, time::Month::September, 22).unwrap(),
                Date::from_calendar_date(2025, time::Month::September, 1).unwrap(),
                
            ],
        }
    }
}
