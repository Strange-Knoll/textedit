use ratatui::{*, prelude::CrosstermBackend, widgets::Paragraph, text::Text};
use crossterm::{*, terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, event::{Event, KeyCode, EnableMouseCapture, DisableMouseCapture}};
use std::{error::Error, io::{Stdout, self}, time::Duration};

mod textedit;
use textedit::TextEdit;


fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, EnableMouseCapture)?; 
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    //buttons need this to be here so that they function properly
    let mut input: Option<Event> = None;
    let mut text_edit_binding = TextEdit::default()
        .text("Hello World!".to_owned());
    let mut text_edit = text_edit_binding
        .cursor(0,0);
    loop {
        if event::poll(Duration::from_millis(100))? {
            input = Some(event::read().unwrap());
        } else {
            input = None;
        }

        match input.clone() {

            Some(Event::Key(key)) => {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
            _ => {}
        }

        terminal.draw(|frame| {
            text_edit.edit(input.clone());
            
            frame.render_widget(text_edit.clone(), frame.size());
        })?;
    }
    Ok(())
}
