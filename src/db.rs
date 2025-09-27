use rusqlite::{Connection, Params, Result};

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
            .prepare("SELECT * FROM habits").expect("idk");
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
            }).unwrap()
            .filter_map(|res| res.ok()).collect();
        // Ok(())
        habit_vec
    }
}

impl Default for db {
    fn default() -> Self {
        db::new()
    }
}
