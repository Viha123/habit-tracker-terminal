use ratatui::{
    widgets::{ListState},
};



#[derive(Debug, Default, Clone)]
pub struct HabitItem {
    // maybe database key to the calendar and bar chart information about a particular habit?
    pub name: String,
}

#[derive(Debug, Default)]
pub struct UserHabits {
    pub show_habit_list: bool,
    pub habit_calendar_track: bool,
    pub habit_stats: bool,
    pub items: Vec<HabitItem>,
    pub state: ListState,
}

impl UserHabits {
    
}
