mod app;
mod date_styler;
mod my_colors;
mod user_habits;
mod db;
mod key_handlers;
mod text_input;
mod input_mode;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal);
    ratatui::restore();
    result
}
