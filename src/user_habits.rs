use ratatui::{
    widgets::{ListState},
};
// struct that saves habit into database schema
#[derive(Debug, Default, Clone)]
pub struct HabitItem {
    // maybe database key to the calendar and bar chart information about a particular habit?
    pub id: u64,
    pub name: String,
    pub active: bool, 
    pub frequency: u32,
    pub current_streak: u32,
    pub max_streak: u32
}
#[derive(Debug, Default, Clone)]
// struct that saves habit into calendar for db schema
pub struct habit_calendar {
    pub id: u64,
    pub date_completed: String,
    pub hours: f32,
    pub notes: String
}


// struct for list UI, etc
#[derive(Debug, Default)]
pub struct UserHabits {
    pub show_habit_list: bool,
    pub show_add_habit: bool,
    pub habit_calendar_track: bool,
    pub habit_stats: bool,
    pub items: Vec<HabitItem>,
    pub state: ListState,
}

impl UserHabits {
    
}
