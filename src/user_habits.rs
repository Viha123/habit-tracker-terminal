use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        Color, Modifier, Style, Stylize,
        palette::tailwind::{BLUE, GREEN, SLATE},
    },
    symbols,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
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
