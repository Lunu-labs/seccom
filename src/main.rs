use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout};
use std::result::Result;
use crossterm::event::{Event, KeyCode, read};

enum EventType {
    TextInput(char),
    ScrollUp,
    ScrollDown,
}

struct LogLine {
    text: String,
}

struct InputEvent {
    event_type: EventType,
}

impl InputEvent {
    fn new(event_type: EventType) -> Self {
        InputEvent { event_type }
    }
}

struct App {
    log: Vec<LogLine>,
    input_buffer: String,
    log_window_start: usize,
}

impl App {
    fn add_log_line(&mut self, text: String) {
        self.log.push(LogLine { text });
    }

    fn process_event(&mut self, event: InputEvent) {
        match event.event_type {
            EventType::TextInput(c) => self.input_buffer.push(c),
            EventType::ScrollUp => {
                if self.log_window_start > 0 {
                    self.log_window_start -= 1;
                }
            }
            EventType::ScrollDown => {
                if self.log_window_start < self.log.len() - 1 {
                    self.log_window_start += 1;
                }
            }
        }
    }

    fn render(&self) {
        execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Show
            ).unwrap();

        let log_end = std::cmp::min(self.log_window_start + 10, self.log.len());
        for log in &self.log[self.log_window_start..log_end] {
            println!("{}", log.text);
        }

        println!("Input: {}", self.input_buffer);
    }
}

// use crossterm::event::{read, Event, KeyCode};

fn main() -> Result<(), io::Error> {
    let mut app = App {
        log: Vec::new(),
        input_buffer: String::new(),
        log_window_start: 0,
    };

    loop {
        app.render();
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    app.process_event(InputEvent::new(EventType::TextInput(c)));
                },
                KeyCode::Up => {
                    app.process_event(InputEvent::new(EventType::ScrollUp));
                },
                KeyCode::Down => {
                    app.process_event(InputEvent::new(EventType::ScrollDown));
                },
                KeyCode::Enter => {
                    app.add_log_line(app.input_buffer.clone());
                    app.input_buffer.clear();
                },
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    Ok(())
}