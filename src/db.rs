use rusqlite::{Connection, Params, Result};
use time::Date;

use crate::user_habits::HabitItem;
#[derive(Debug)]
pub struct db {
    pub conn: Result<Connection>,
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
                Ok(HabitItem {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    active: row.get(2)?,
                    frequency: row.get(3)?,
                    current_streak: row.get(4)?,
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
        let _res = self.conn.as_ref().unwrap().execute("INSERT INTO habit_calendar(habit_id, date_completed, hours, notes) VALUES (?1, ?2, ?3, ?4)", (item.id, date.to_string(), hours, "Nothing"));
        
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
    pub fn compute_streak() {
        todo!()
    }

    pub fn delete_habit(&self, habit_id: u64) -> rusqlite::Result<usize> {
        let conn = self.conn.as_ref().map_err(|e| rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
            Some("Database connection failed".to_string())
        ))?;
        
        let rows_affected = conn.execute(
            "DELETE FROM habits WHERE habit_id = ?1",
            [habit_id],
        )?;
        
        Ok(rows_affected)
    }
}

impl Default for db {
    fn default() -> Self {
        db::new()
    }
}
