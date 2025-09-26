use rusqlite::{Params, Connection, Result};

use crate::user_habits::HabitItem;


pub fn add_habit(habit: &HabitItem) -> Result<()>{
  let conn = Connection::open("../habit-tracker.db")?;
  
  Ok(())
}