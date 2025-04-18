use color_eyre::eyre::Ok;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{self, Line, Span},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

use crate::buffer::buffer::Buffer;

pub struct App {
    running: bool,
    buffer: Buffer,
    cursor_position: (usize, usize),
}

impl App {
    pub fn new(buffer: Buffer) -> Self {
        App {
            running: true,
            buffer,
            cursor_position: (0, 0),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        while self.running {
            terminal.draw(|frame| self.draw(frame));
            self._events();
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        let mut layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80)])
            .split(frame.area());
        let mut text: Vec<Line> = self
            .buffer
            .rows
            .iter()
            .map(|row| Line::from(Span::raw(row)))
            .collect();
        let mut text: Vec<Line> = Vec::new();
        for (i, row) in self.buffer.rows.iter().enumerate() {
            if (i == self.cursor_position.0) {
                let mut spans: Vec<Span> = Vec::new();
                for (j, c) in self.buffer.rows.get(i).iter().enumerate() {
                    if (j == self.cursor_position.1) {
                        spans.push(Span::raw(**c.clone()).bg(Color::Red));
                    } else {
                    }
                }
            } else {
                text.push(Line::from(Span::raw(row)))
            }
        }
        frame.render_widget(Paragraph::new(text), layout[1]);
    }

    pub fn _events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.running = false,
                    (_) => {}
                }
            }
        }
        Ok(())
    }
}
