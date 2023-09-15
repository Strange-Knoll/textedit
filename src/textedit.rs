use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::{StatefulWidget, Paragraph, Widget, Wrap}};

#[derive(Clone)]
pub struct TextEdit{
    text: String,
    cursor: (usize, usize),
}

impl Default for TextEdit{
    fn default() -> Self{
        Self{
            text: String::new(),
            cursor: (0,0),
        }
    }
}

impl TextEdit{
    pub fn text(&mut self, text: &str) -> TextEdit{
        self.text = text.to_string();
        self.clone()
    }
    pub fn cursor(&mut self, cursor_x: usize, cursor_y: usize) -> TextEdit{
        self.cursor = (cursor_x, cursor_y);
        self.clone()
    }

    pub fn edit(&mut self, state: Option<Event>){
        match state{
            Some(Event::Key(key)) => {
                match key.code{
                    KeyCode::Char(c) => {
                        self.text.insert(self.cursor.0, c);
                        self.cursor.0 += 1;
                    }
                    KeyCode::Enter => {
                        self.text.insert_str(self.cursor.0, " \n ");
                        self.cursor.0 += 1;
                    }
                    KeyCode::Backspace => {
                        if self.cursor.0 > 0{
                            self.text.remove(self.cursor.0 - 1);
                            self.cursor.0 -= 1;
                        }
                    }
                    KeyCode::Delete => {
                        if self.cursor.0 < self.text.len(){
                            self.text.remove(self.cursor.0);
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor.0 > 0{
                            self.cursor.0 -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor.0 < self.text.len(){
                            self.cursor.0 += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Widget for TextEdit{
    fn render(self, area: Rect, buf: &mut Buffer) {
    
        let mut text = Text::from(self.text.clone());
        let line_at_cursor = &text.lines[self.cursor.1];
        let mut string_from_spans = String::new();
        for spans in line_at_cursor.spans.clone(){
            string_from_spans.push_str(&spans.content.clone());
        };
        let text_before_cursor = &string_from_spans[..self.cursor.0];
        //let text_before_cursor = &self.text[..self.cursor.0];
        let text_after_cursor = &string_from_spans[ 
            if self.cursor.0+1 < string_from_spans.len(){
                self.cursor.0+1
            }else{
                string_from_spans.len()
            }
            ..];
        let text_at_cursor = &self.text.chars().nth(self.cursor.0).unwrap_or(' ');

        let cursor_span = Span::styled(text_at_cursor.to_string(), Style::default().fg(Color::Black).bg(Color::White));
        let text_before_cursor_span = Span::raw(text_before_cursor);
        let text_after_cursor_span = Span::raw(text_after_cursor);

        let line = Line::from(vec![
            text_before_cursor_span,
            cursor_span,
            text_after_cursor_span,
        ]);
        text.lines[self.cursor.1] = line;

        let text = Paragraph::new(text)
            .wrap(Wrap { trim: false });
        text.render(area, buf)
    }
}
