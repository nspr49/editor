use std::fs;

use buffer::parse;
use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event};
use draw::app::App;
use ratatui::{DefaultTerminal, Frame};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filename: String,
    #[arg(short, long, default_value_t = String::from("normal"))]
    mode: String,
}

mod buffer;
mod draw;

/*
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
*/

fn main() {
    let args = Args::parse();
    let buffer = parse::parse(args.filename);
    let terminal = ratatui::init();
    let app_result = App::new(buffer).run(terminal);
    ratatui::restore();
    app_result
}
