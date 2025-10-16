use crossterm::event::KeyCode;

#[derive(Default, Debug)]
pub struct TextInput {
    pub content: String,
    pub cursor_position: usize,
}

impl TextInput {
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.content.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.content.remove(self.cursor_position);
                }
            }
            KeyCode::Delete => {
                if self.cursor_position < self.content.len() {
                    self.content.remove(self.cursor_position);
                }
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_position < self.content.len() {
                    self.cursor_position += 1;
                }
            }
            KeyCode::Esc | KeyCode::BackTab => {
                self.content.clear();
                self.cursor_position = 0;

            }
            KeyCode::Home => self.cursor_position = 0,
            KeyCode::End => self.cursor_position = self.content.len(),
            _ => {}
        }
    }
}
