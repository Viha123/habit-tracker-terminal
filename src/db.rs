use rusqlite::{Connection, Params, Result};
use std::fmt;
use time::Date;

use crate::user_habits::HabitItem;
#[derive(Debug)]
pub struct db {
    pub conn: Result<Connection>,
}
pub enum TimeFrame {
    Week,
    Month,
    Year,
}
impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeFrame::Week => write!(f, "Week"),
            TimeFrame::Month => write!(f, "Month"),
            TimeFrame::Year => write!(f, "Year"),
        }
    }
}

impl db {
    pub fn new() -> Self {
        Self {
            conn: match Connection::open("habit-tracker.db") {
                Ok(c) => Ok(c),
                Err(e) => {
                    eprint!("Failed to connect to database");
                    Err(e)
                }
            },
        }
    }
    pub fn add_habit(&self, name: &str, frequency: &u32) -> Result<(HabitItem)> {
        self.conn.as_ref().unwrap().execute(
            "
          INSERT INTO habits(name, active, frequency, current_streak, max_streak) 
          VALUES (?1, ?2, ?3, ?4, ?5)",
            (name, true, frequency, 0, 0),
        )?; // this is a new habit. 
        let mut stmt = self
            .conn
            .as_ref()
            .unwrap()
            .prepare("SELECT * FROM habits where name = (?1)")?;
        stmt.execute([name])?;
        let habit = stmt.query_row([], |row| {
            Ok(HabitItem {
                id: row.get(0)?,
                name: row.get(1)?,
                active: row.get(2)?,
                frequency: row.get(3)?,
                current_streak: row.get(4)?,
                max_streak: row.get(5)?,
            })
        })?;

        return Ok(habit);
    }

    pub fn get_habits(&self) -> Vec<HabitItem> {
        let mut stmt = self
            .conn
            .as_ref()
            .expect("should be a connection")
            .prepare("SELECT * FROM habits")
            .expect("idk");
        let habit_vec: Vec<HabitItem> = stmt
            .query_map([], |row| {
                let id: u64 = row.get(0)?;
                let frequency: u32 = row.get(3)?;
                let computed_streak = self.compute_streak(id, frequency);

                Ok(HabitItem {
                    id,
                    name: row.get(1)?,
                    active: row.get(2)?,
                    frequency,
                    current_streak: computed_streak,
                    max_streak: row.get(5)?,
                })
            })
            .unwrap()
            .filter_map(|res| res.ok())
            .collect();
        // Ok(())
        habit_vec
    }

    pub fn add_completed(&self, date: &Date, item: &HabitItem, hours: u32) {
        let _res = self.conn.as_ref().unwrap().execute(
            "INSERT INTO habit_calendar(habit_id, date_completed, hours) 
         VALUES (?1, ?2, ?3)
         ON CONFLICT(habit_id, date_completed) 
         DO UPDATE SET hours = ?3",
            (item.id, date.to_string(), hours),
        );
    }

    pub fn get_id_from_name(&self, name: String) -> u64 {
        let mut stmt = self
            .conn
            .as_ref()
            .expect("should be a connection")
            .prepare("SELECT habit_id from habits where name = (?1)")
            .expect("could not get id from name");
        let id: Vec<u64> = stmt
            .query_map([name], |row| Ok(row.get(0)?))
            .unwrap()
            .filter_map(|res| res.ok())
            .collect();

        return id[0];
    }

    pub fn list_completed_dates(&self, id: u64) -> Vec<String> {
        let mut stmt = self
            .conn
            .as_ref()
            .expect("Connection refused")
            .prepare("SELECT date_completed from habit_calendar WHERE habit_id = (?1)")
            .expect("wrong sql prep");
        let dates_vec: Vec<String> = stmt
            .query_map([id], |row| Ok(row.get(0)?))
            .unwrap()
            .filter_map(|res| res.ok())
            .collect();
        dates_vec
    }

    pub fn list_streak_dates(&self, id: u64, frequency: u32, current_streak: u32) -> Vec<String> {
        // If streak is <= 5, return empty vec (no orange highlighting)
        if current_streak <= 5 {
            return Vec::new();
        }

        // Get all completed dates ordered descending
        let mut stmt = self
            .conn
            .as_ref()
            .expect("Connection refused")
            .prepare("SELECT date_completed FROM habit_calendar WHERE habit_id = (?1) ORDER BY date_completed DESC")
            .expect("wrong sql prep");

        let dates_vec: Vec<String> = stmt
            .query_map([id], |row| Ok(row.get(0)?))
            .unwrap()
            .filter_map(|res| res.ok())
            .collect();

        // Parse and filter to get only the dates that are part of the current streak
        let parsed_dates: Vec<Date> = dates_vec
            .iter()
            .filter_map(|date_str| {
                Date::parse(
                    date_str,
                    &time::format_description::well_known::Iso8601::DEFAULT,
                )
                .ok()
            })
            .collect();

        if parsed_dates.is_empty() {
            return Vec::new();
        }

        let today = time::OffsetDateTime::now_utc().date();
        let mut streak_dates = Vec::new();
        let mut expected_date = today;

        // Only include dates that are part of the active streak
        for completed_date in parsed_dates {
            let gap = (expected_date - completed_date).whole_days();

            if gap >= 0 && gap <= frequency as i64 {
                streak_dates.push(completed_date.to_string());
                expected_date = completed_date - time::Duration::days(frequency as i64);
            } else {
                break;
            }
        }

        streak_dates
    }
    pub fn compute_streak(&self, habit_id: u64, frequency: u32) -> u32 {
        // Get all completed dates for this habit, ordered by date descending
        let mut stmt = self
            .conn
            .as_ref()
            .expect("Connection refused")
            .prepare("SELECT date_completed FROM habit_calendar WHERE habit_id = (?1) ORDER BY date_completed DESC")
            .expect("wrong sql prep");

        let dates_vec: Vec<String> = stmt
            .query_map([habit_id], |row| Ok(row.get(0)?))
            .unwrap()
            .filter_map(|res| res.ok())
            .collect();

        if dates_vec.is_empty() {
            return 0;
        }

        // Parse dates and calculate streak
        let parsed_dates: Vec<Date> = dates_vec
            .iter()
            .filter_map(|date_str| {
                Date::parse(
                    date_str,
                    &time::format_description::well_known::Iso8601::DEFAULT,
                )
                .ok()
            })
            .collect();

        if parsed_dates.is_empty() {
            return 0;
        }

        let today = time::OffsetDateTime::now_utc().date();
        let mut streak = 0u32;
        let mut expected_date = today;

        // Check if the most recent completion is within the frequency window
        let days_since_last = (today - parsed_dates[0]).whole_days();
        if days_since_last > frequency as i64 {
            // Streak is broken
            return 0;
        }

        // Count consecutive completions within frequency windows
        for completed_date in parsed_dates {
            let gap = (expected_date - completed_date).whole_days();

            if gap >= 0 && gap <= frequency as i64 {
                streak += 1;
                expected_date = completed_date - time::Duration::days(frequency as i64);
            } else {
                break;
            }
        }
        streak
    }

    pub fn get_hours(&self, habit_id: u64, tf: TimeFrame) -> u32 {
        // takes in a time frame and returns habit hours for that timeframe
        let date_str;
        match tf {
            TimeFrame::Month => date_str = "%Y-%m",
            TimeFrame::Week => {
                date_str = "%Y-%W";
            }
            TimeFrame::Year => {
                date_str = "%Y";
            }
        }
        let mut stmt = self.conn.as_ref().expect("Connection Refused").prepare("select SUM(hours) from habit_calendar where strftime((?1), date_completed) = strftime((?1), 'now') AND habit_id=(?2);").expect("wrong sql get date stuff");
        let hours: Option<u32> = stmt
            .query_row((date_str, habit_id), |row| row.get(0))
            .unwrap_or(None);
        hours.unwrap_or(0)
    }

    pub fn delete_habit(&self, habit_id: u64) -> rusqlite::Result<usize> {
        let conn = self.conn.as_ref().map_err(|_| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                Some("Database connection failed".to_string()),
            )
        })?;

        let rows_affected = conn.execute("DELETE FROM habits WHERE habit_id = (?1)", [habit_id])?;

        Ok(rows_affected)
    }
}

impl Default for db {
    fn default() -> Self {
        db::new()
    }
}
