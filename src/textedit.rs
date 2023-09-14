use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::{StatefulWidget, Paragraph, Widget}};

#[derive(Clone)]
pub struct TextEdit<'a>{
    text: Text<'a>,
    cursor: usize,
}

impl Default for TextEdit<'_>{
    fn default() -> Self{
        Self{
            text: Text::from(""),
            cursor: 0,
        }
    }
}

impl<'a> TextEdit<'a>{
    pub fn text(&mut self, text: Text<'a>) -> TextEdit<'a>{
        self.text = text;
        self.clone()
    }
    pub fn cursor(&mut self, cursor: usize) -> TextEdit<'a>{
        self.cursor = cursor;
        self.clone()
    }

    pub fn edit(&mut self, state: Option<Event>){
        match state{
            Some(Event::Key(key)) => {
                match key.code{
                    KeyCode::Char(c) => {
                        self.text.lines.insert(self.cursor, Line::from(c.to_string()));
                        self.cursor += 1;
                    }
                    KeyCode::Backspace => {
                        if self.cursor > 0{
                            self.text.lines.remove(self.cursor - 1);
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Delete => {
                        if self.cursor < self.text.lines.len(){
                            self.text.lines.remove(self.cursor);
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor > 0{
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor < self.text.lines.len(){
                            self.cursor += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Widget for  TextEdit<'_>{
    fn render(self, area: Rect, buf: &mut Buffer) {
        

        let text = Paragraph::new(self.text);
        text.render(area, buf)
    }
}