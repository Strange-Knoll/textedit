use std::io::BufRead;

use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::{StatefulWidget, Paragraph, Widget, Wrap}};


fn span_content_from_line(line:Line) -> String{
    let mut text = String::new();
    for span in line.spans{
        text.push_str(&span.content);
    }
    text
}

fn what_span_am_i_in(line:Line, cursor_x: usize) -> usize{ 
    let spans = line.spans.clone();
    let mut sum_width = 0;
    for (i, span) in spans.iter().enumerate(){
        sum_width += span.width();
        if sum_width > cursor_x{
            return i;
        }
    }
    return spans.len() - 1;
} 


#[derive(Clone)]
pub struct TextEdit<'a>{
    text: Text<'a>,
    cursor: (usize, usize),
}

impl Default for TextEdit<'_>{
    fn default() -> Self{
        Self{
            text: Text::default(),
            cursor: (0,0),
        }
    }
}

impl<'a> TextEdit<'a>{
    pub fn text(&mut self, text: String) -> TextEdit<'a>{
        self.text = Text::from(text);
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
                        let spans = self.text.lines[self.cursor.1].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }
                        text.insert(self.cursor.0, c);
                        self.text.lines[self.cursor.1].spans = vec![Span::raw(text)];
                        self.cursor.0 += 1;
                    }
                    KeyCode::Enter => {
                        let spans = self.text.lines[self.cursor.1].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }
                        let text_before_cursor = text[..self.cursor.0].to_string();
                        let text_after_cursor = text[self.cursor.0..].to_string();
                        self.text.lines[self.cursor.1] = Line::from(vec![Span::raw(text_before_cursor)]);
                        self.text.lines.insert(self.cursor.1 + 1, 
                            if text_after_cursor.len() > 0{ 
                                Line::from(vec![Span::raw(text_after_cursor)])
                            }else{
                                Line::from(vec![Span::raw(" ")])
                            }
                        );
                        self.cursor.1 += 1;
                        self.cursor.0 = 0;
                    }
                    KeyCode::Backspace => {
                        let spans = self.text.lines[self.cursor.1].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }


                        if self.cursor.0 > 0{
                            text.remove(self.cursor.0 - 1);
                            self.text.lines[self.cursor.1].spans = vec![Span::raw(text)];
                            self.cursor.0 -= 1;
                        }else{
                            let mut text = String::new();
                            for span in self.text.lines[self.cursor.1].spans.clone(){
                                text.push_str(&span.content);
                            }
                            let line_after_cursor = text[self.cursor.0..].to_string();
                            self.text.lines.remove(self.cursor.1);
                            self.cursor.1 -= 1;
                            self.cursor.0 = self.text.lines[self.cursor.1].width();
                            let mut text = self.text.lines[self.cursor.1].spans.clone();
                            text.append(&mut vec![Span::raw(line_after_cursor)]);
                            self.text.lines[self.cursor.1].spans = text;
                        }
                    }
                    KeyCode::Delete => {
                        let spans = self.text.lines[self.cursor.1].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }


                        if self.cursor.0 < text.len(){
                            text.remove(self.cursor.0);
                            self.text.lines[self.cursor.1].spans = vec![Span::raw(text)];
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor.0 > 0{
                            self.cursor.0 -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor.0 < self.text.lines[self.cursor.1].width(){
                            self.cursor.0 += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.cursor.1 > 0{
                            let spans = self.text.lines[self.cursor.1-1].spans.clone();
                            let mut text = String::new();
                            for span in spans{
                                text.push_str(&span.content);
                            }
                            if self.cursor.0 > text.len(){
                                self.cursor.0 = text.len();
                            }
                            self.cursor.1 -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.cursor.1 < self.text.lines.len() - 1{
                            self.cursor.1 += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Widget for TextEdit<'_>{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut text_obj = self.text;
        let line = text_obj.lines[self.cursor.1].clone();
        let mut text = String::new();
        for span in line.spans{
            text.push_str(&span.content);
        }
        let span_at_cursor = Span::styled(
            text.chars().nth(self.cursor.0).unwrap_or(' ').to_string(), 
            Style::default().fg(Color::Black).bg(Color::Gray));
        let span_before_cursor = Span::raw(text[..self.cursor.0].to_string());
        let span_after_cursor = Span::raw(text[
            if self.cursor.0 < text.len(){
                self.cursor.0 + 1
            }else{
                self.cursor.0
            }..].to_string());
        text_obj.lines[self.cursor.1].spans = vec![span_before_cursor, span_at_cursor, span_after_cursor];

        let text = Paragraph::new(text_obj)
            .wrap(Wrap { trim: false });
        text.render(area, buf)
    }
}
