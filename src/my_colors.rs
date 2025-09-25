use catppuccin::PALETTE;
use ratatui::style::{Color, Modifier, Style};


pub const NORMAL_ROW_BG: Color = convert_color_type(PALETTE.macchiato.colors.base);
pub const BORDER_COL: Color = convert_color_type(PALETTE.macchiato.colors.flamingo);
pub const ALT_ROW_BG_COLOR: Color = convert_color_type(PALETTE.macchiato.colors.crust);
pub const SELECTED_STYLE: Style = Style::new()
    .fg(convert_color_type(PALETTE.macchiato.colors.green))
    .add_modifier(Modifier::BOLD);

const fn convert_color_type(color: catppuccin::Color) -> Color {
    return Color::Rgb(color.rgb.r, color.rgb.g, color.rgb.b);
}