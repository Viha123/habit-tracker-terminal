use crate::{db::db, my_colors};
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
            completed_dates: vec![],
        }
    }
    // this is technically inefficient but we really don't care for like habits and such
    pub fn update_dates(&mut self, dates: Vec<String>) -> Result<(), time::Error> {
        self.completed_dates.clear();
        let format = time::format_description::parse("[year]-[month]-[day]")?;
        for date_str in dates {
            let date = Date::parse(&date_str, &format)?;
            self.completed_dates.push(date);
        }

        Ok(())
    }
}
