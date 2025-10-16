use ratatui::{style::Style, widgets::calendar::DateStyler};
use time::Date;

use crate::my_colors::{SELECTED_STYLE, STREAK_STYLE};

#[derive(Debug, Default)]
pub struct CompletedDateStyler {
    pub completed_dates: Vec<Date>,
    pub streak_dates: Vec<Date>,
}

impl CompletedDateStyler {
    pub fn new() -> Self {
        CompletedDateStyler {
            completed_dates: Vec::new(),
            streak_dates: Vec::new(),
        }
    }
    pub fn update_dates(&mut self, dates: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        for date in dates {
            let d = Date::parse(
                &date,
                &time::format_description::well_known::Iso8601::DEFAULT,
            )?;
            self.completed_dates.push(d);
        }
        Ok(())
    }

    pub fn update_streak_dates(
        &mut self,
        dates: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for date in dates {
            let d = Date::parse(
                &date,
                &time::format_description::well_known::Iso8601::DEFAULT,
            )?;
            self.streak_dates.push(d);
        }
        Ok(())
    }
}
impl DateStyler for CompletedDateStyler {
    fn get_style(&self, date: Date) -> Style {
        if self.streak_dates.contains(&date) {
            return STREAK_STYLE;
        }
        if self.completed_dates.contains(&date) {
            return SELECTED_STYLE;
        }
        Style::default()
    }
}
