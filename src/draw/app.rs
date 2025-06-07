use std::io::{self, stdout, Write};

use color_eyre::Result;
use color_eyre::{eyre::Ok, owo_colors::OwoColorize};
use crossterm::cursor::{EnableBlinking, RestorePosition, SetCursorStyle};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{execute, ExecutableCommand};
use ratatui::layout::{self, Position, Rect};
use ratatui::style::Style;
use ratatui::{
    layout::{Constraint, Layout},
    text::{self, Line, Span},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

use crate::buffer::buffer::Buffer;

#[derive(PartialEq, Eq)]
pub enum EditMode {
    NORMAL,
    INSERT,
    COMMAND,
}

pub struct App {
    running: bool,
    file_name: String,
    buffer: Buffer,
    cursor_position: (usize, usize),
    mode: EditMode,
    margin_top: u16,
    bottom_bar: u16,
}

impl App {
    pub fn new(buffer: Buffer, file_name: String) -> Self {
        App {
            running: true,
            file_name,
            buffer,
            cursor_position: (0, 0),
            mode: EditMode::NORMAL,
            margin_top: 0,
            bottom_bar: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        while self.running {
            terminal.draw(|frame| self.draw(frame));
            self.handle_events();
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(frame.area());
        let mut text: Vec<Line> = Vec::new();
        for (i, row) in self.buffer.rows.iter().enumerate() {
            text.push(Line::from(Span::raw(row)))
        }
        frame.render_widget(Paragraph::new(text), layout[1]);
        frame.set_cursor_position(Position {
            x: self.cursor_position.0 as u16,
            y: self.cursor_position.1 as u16 + layout[0].height,
        });
        self.margin_top = layout[0].height;
        self.bottom_bar = layout[2].height;
        self.draw_bar(frame, layout[2]);
    }

    pub fn draw_bar(&mut self, frame: &mut Frame, layout: Rect) {
        match self.mode {
            EditMode::NORMAL => {
                frame.render_widget(Paragraph::new("Normal"), layout);
            }
            EditMode::INSERT => {
                frame.render_widget(
                    Paragraph::new("Insert").style(Style::new().fg(ratatui::style::Color::Green)),
                    layout,
                );
            }
            EditMode::COMMAND => {
                frame.render_widget(
                    Paragraph::new("Command").style(Style::new().fg(ratatui::style::Color::Red)),
                    layout,
                );
            }
        }
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if self.mode == EditMode::NORMAL {
            self.handle_events_normal()?;
        } else if self.mode == EditMode::INSERT {
            self.handle_events_insert()?;
        } else {
            self.handle_events_command()?;
        }
        Ok(())
    }

    pub fn handle_events_normal(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('j') => {
                        if self.buffer.rows.len() - 1 > self.cursor_position.1 {
                            self.cursor_position.1 += 1;

                            if self.buffer.rows.get(self.cursor_position.1).unwrap().len()
                                < self.cursor_position.0
                            {
                                self.cursor_position.0 =
                                    self.buffer.rows.get(self.cursor_position.1).unwrap().len();
                            }
                        }
                    }
                    KeyCode::Char('k') => {
                        if self.cursor_position.1 > 0 {
                            self.cursor_position.1 -= 1;
                            if self.buffer.rows.get(self.cursor_position.1).unwrap().len()
                                < self.cursor_position.0
                            {
                                if self.buffer.rows.get(self.cursor_position.1).unwrap().len() == 0
                                {
                                    self.cursor_position.0 = 0;
                                } else {
                                    self.cursor_position.0 =
                                        self.buffer.rows.get(self.cursor_position.1).unwrap().len()
                                            - 1;
                                }
                            }
                        }
                    }
                    KeyCode::Char('l') => {
                        if self
                            .buffer
                            .rows
                            .get(self.cursor_position.1)
                            .expect("Failed to get line")
                            .len()
                            > 0
                            && self
                                .buffer
                                .rows
                                .get(self.cursor_position.1)
                                .expect("Failed to get line")
                                .len()
                                - 1
                                > self.cursor_position.0
                        {
                            self.cursor_position.0 += 1
                        }
                    }
                    KeyCode::Char('h') => {
                        if self.cursor_position.0 > 0 {
                            self.cursor_position.0 -= 1
                        }
                    }
                    KeyCode::Char('i') => {
                        io::stdout().execute(SetCursorStyle::BlinkingBar)?;
                        self.mode = EditMode::INSERT;
                    }
                    KeyCode::Char('a') => {
                        if self.buffer.rows.len() > self.cursor_position.0 + 1 {
                            self.cursor_position.0 += 1;
                        }
                        io::stdout().execute(SetCursorStyle::BlinkingBar)?;
                        self.mode = EditMode::INSERT;
                    }
                    KeyCode::Char(':') => {
                        io::stdout().execute(SetCursorStyle::SteadyUnderScore)?;
                        self.mode = EditMode::COMMAND;
                    }
                    KeyCode::Char('$') => {
                        self.cursor_position.0 =
                            self.buffer.rows.get(self.cursor_position.1).unwrap().len() - 1;
                    }
                    KeyCode::Char('^') => {
                        self.cursor_position.0 = 0;
                    }

                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn handle_events_insert(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = EditMode::NORMAL;
                        io::stdout().execute(SetCursorStyle::DefaultUserShape)?;
                    }
                    KeyCode::Enter => {
                        if self.cursor_position.1 < self.buffer.rows.len() {
                            self.buffer
                                .rows
                                .insert(self.cursor_position.1 + 1, String::new());
                            self.cursor_position.1 += 1;
                            self.cursor_position.0 = 0;
                        } else {
                            self.buffer.rows.push(String::new());
                            self.cursor_position.1 += 1;
                            self.cursor_position.0 = 0;
                        }
                    }
                    KeyCode::Char(c) => {
                        if let Some(row) = self.buffer.rows.get_mut(self.cursor_position.1) {
                            row.insert(self.cursor_position.0, c);
                            self.cursor_position.0 += 1;
                        }
                    }
                    KeyCode::Backspace => {
                        self.buffer
                            .rows
                            .get_mut(self.cursor_position.1)
                            .unwrap()
                            .remove(self.cursor_position.0);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn handle_events_command(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.running = false,
                    KeyCode::Char('w') => {
                        let mut file = std::fs::File::create(&self.file_name)?;
                        let file_content = self
                            .buffer
                            .rows
                            .iter()
                            .map(|s| s.to_string())
                            .reduce(|x: String, y: String| x + "\n" + &y)
                            .unwrap()
                            .into_bytes();
                        // TODO handle error with message
                        file.write_all(&file_content).unwrap();
                        io::stdout().execute(SetCursorStyle::DefaultUserShape);
                        self.mode = EditMode::NORMAL;
                    }
                    KeyCode::Esc => {
                        self.mode = EditMode::NORMAL;
                        io::stdout().execute(SetCursorStyle::DefaultUserShape);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
