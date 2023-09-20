use std::io::BufRead;

use crossterm::event::{Event, KeyCode};
use ratatui::{prelude::*, widgets::{StatefulWidget, Paragraph, Widget, Wrap}, symbols::line};

#[derive(Clone)]
struct Cursor {
    x: usize,
    y: usize,
}

fn line_to_string(line:Line) -> String{
    let mut text = String::new();
    for span in line.spans{
        text.push_str(&span.content);
    }
    text
}

struct SpanInLineIndex(usize); //what span in the line am i in
struct CharInSpanIndex(usize); //where in the span am i   

fn where_am_i(line:Line, cursor: Cursor) -> (SpanInLineIndex, CharInSpanIndex){ 
    let spans = line.spans.clone();
    let mut sum_width = 0;
    for (i, span) in spans.iter().enumerate(){
        if sum_width > cursor.x{
            return (SpanInLineIndex(i), CharInSpanIndex(cursor.x - sum_width));
        }else{
            sum_width += span.content.len();
        }
    }
    return (SpanInLineIndex(spans.len() - 1), CharInSpanIndex(cursor.x - sum_width));
}

fn split_span(line:Line, location:(SpanInLineIndex, CharInSpanIndex)) -> (Span, Span){
    let span_index = location.0.0;
    let char_index = location.1.0;
    let span = line.spans[span_index].clone();
    let mut text = span.content.clone();

    let text_before_cursor = text[..char_index].to_string();
    let text_after_cursor = text[
        if char_index<text.len(){
            char_index + 1
        }else{
            char_index
        }..].to_string();
    
    (
        Span::styled(text_before_cursor, span.style.clone()),
        Span::styled(text_after_cursor, span.style.clone())
    )
    
}

fn split_span_at_cursor(line:Line, location:(SpanInLineIndex, CharInSpanIndex)) -> (Span, Span, Span){
    let span_index = location.0.0;
    let char_index = location.1.0;
    let span = line.spans[span_index].clone();
    let mut text = span.content.clone();

    let text_before_cursor = text[..char_index].to_string();
    let text_at_cursor = text.chars().nth(char_index).unwrap_or(' ').to_string();
    let text_after_cursor = text[
        if char_index<text.len(){
            char_index + 1
        }else{
            char_index
        }..].to_string();
    
    (
        Span::styled(text_before_cursor, span.style.clone()),
        Span::styled(text_at_cursor, Style::default().fg(Color::Black).bg(Color::Gray)),
        Span::styled(text_after_cursor, span.style.clone())
    )
    
}


#[derive(Clone)]
pub struct TextEdit<'a>{
    text: Text<'a>,
    cursor: Cursor,
}

impl Default for TextEdit<'_>{
    fn default() -> Self{
        Self{
            text: Text::default(),
            cursor: Cursor{x: 0, y: 0}
        }
    }
}

impl<'a> TextEdit<'a>{
    pub fn text(&mut self, text: String) -> TextEdit<'a>{
        self.text = Text::from(text);
        self.clone()
    }
    pub fn cursor(&mut self, x: usize, y: usize) -> TextEdit{
        self.cursor = Cursor{x: x, y: y};
        self.clone()
    }

    pub fn edit(&mut self, state: Option<Event>){
        match state{
            Some(Event::Key(key)) => {
                match key.code{
                    KeyCode::Char(c) => {
                        let mut line = self.text.lines[self.cursor.y].clone();
                        //to do this we need to insert a char in the span at the cursor
                        // use where_am_i to find the span and char index
                        let (span_index, char_index) = where_am_i(
                            line.clone(), 
                            self.cursor.clone());
                        let span = line.spans[span_index.0].clone();
                        span.content.to_string().insert(char_index.0, c);
                        line.spans[span_index.0] = span;
                        self.text.lines[self.cursor.y] = line;
                        self.cursor.x += 1;
                        
                        //old
                        //let spans = self.text.lines[self.cursor.y].spans.clone();
                        //let mut text = String::new();
                        //for span in spans{
                        //    text.push_str(&span.content);
                        //}
                        //text.insert(self.cursor.x, c);
                        //self.text.lines[self.cursor.y].spans = vec![Span::raw(text)];
                        //self.cursor.x += 1;
                    }
                    KeyCode::Enter => {
                        // to do this we need to split the span at the cursor
                        // and insert a new line with the span after the cursor
                        // if we are at the end of the line we insert a new line
                        // with a new span and a space

                        let line = self.text.lines[self.cursor.y].clone();
                        let (span_index, char_index) = where_am_i(
                            line.clone(), 
                            self.cursor.clone());

                        let mut spans_before_cursor = line.clone().spans[..span_index.0].to_vec();
                        let mut spans_after_cursor = line.clone().spans[span_index.0..].to_vec();
                        
                        //Problem: this would split the line at the span not the cursor
                        let (left_span, right_span) = split_span(line, (span_index, char_index));
                        spans_before_cursor.append(&mut vec![left_span]);
                        spans_after_cursor.insert(0, right_span);
                        self.text.lines[self.cursor.y] = Line::from(spans_before_cursor);
                        self.text.lines.insert(self.cursor.y + 1, spans_after_cursor.into());
                        
                        self.cursor.y += 1;
                        self.cursor.x = 0;


                        /* let spans = self.text.lines[self.cursor.y].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }
                        let text_before_cursor = text[..self.cursor.x].to_string();
                        let text_after_cursor = text[self.cursor.x..].to_string();
                        self.text.lines[self.cursor.y] = Line::from(vec![Span::raw(text_before_cursor)]);
                        self.text.lines.insert(self.cursor.y + 1, 
                            if text_after_cursor.len() > 0{ 
                                Line::from(vec![Span::raw(text_after_cursor)])
                            }else{
                                Line::from(vec![Span::raw(" ")])
                            }
                        );
                        self.cursor.y += 1;
                        self.cursor.x = 0; */
                    }
                    KeyCode::Backspace => {
                        // to do this we need to remove a char in the span at the cursor
                        // we need to know what span we are in, if we delete the final char
                        // in a span we need to start removing from the next span before

                        /* let spans = self.text.lines[self.cursor.y].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }


                        if self.cursor.x > 0{
                            text.remove(self.cursor.x - 1);
                            self.text.lines[self.cursor.y].spans = vec![Span::raw(text)];
                            self.cursor.x -= 1;
                        }else{
                            let mut text = String::new();
                            for span in self.text.lines[self.cursor.y].spans.clone(){
                                text.push_str(&span.content);
                            }
                            let line_after_cursor = text[self.cursor.x..].to_string();
                            self.text.lines.remove(self.cursor.y);
                            self.cursor.y -= 1;
                            self.cursor.x = self.text.lines[self.cursor.y].width();
                            let mut text = self.text.lines[self.cursor.y].spans.clone();
                            text.append(&mut vec![Span::raw(line_after_cursor)]);
                            self.text.lines[self.cursor.y].spans = text;
                        } */
                    }
                    KeyCode::Delete => {
                        let spans = self.text.lines[self.cursor.y].spans.clone();
                        let mut text = String::new();
                        for span in spans{
                            text.push_str(&span.content);
                        }


                        if self.cursor.x < text.len(){
                            text.remove(self.cursor.x);
                            self.text.lines[self.cursor.y].spans = vec![Span::raw(text)];
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor.x > 0{
                            self.cursor.x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor.x < self.text.lines[self.cursor.y].width(){
                            self.cursor.x += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.cursor.y > 0{
                            let spans = self.text.lines[self.cursor.y-1].spans.clone();
                            let mut text = String::new();
                            for span in spans{
                                text.push_str(&span.content);
                            }
                            if self.cursor.x > text.len(){
                                self.cursor.x = text.len();
                            }
                            self.cursor.y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.cursor.y < self.text.lines.len() - 1{
                            self.cursor.y += 1;
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
        let line = text_obj.lines[self.cursor.y].clone();
        let mut text = String::new();
        for span in line.spans{
            text.push_str(&span.content);
        }
        let span_at_cursor = Span::styled(
            text.chars().nth(self.cursor.x).unwrap_or(' ').to_string(), 
            Style::default().fg(Color::Black).bg(Color::Gray));
        let span_before_cursor = Span::raw(text[..self.cursor.x].to_string());
        let span_after_cursor = Span::raw(text[
            if self.cursor.x < text.len(){
                self.cursor.x + 1
            }else{
                self.cursor.x
            }..].to_string());
        text_obj.lines[self.cursor.y].spans = vec![span_before_cursor, span_at_cursor, span_after_cursor];

        let text = Paragraph::new(text_obj)
            .wrap(Wrap { trim: false });
        text.render(area, buf)
    }
}
