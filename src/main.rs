use std::{io, time::Duration};

mod widgets;

use ratatui::{
    crossterm::event::{self, poll, KeyCode, KeyEventKind},
    // style::Stylize,
    // widgets::Paragraph,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    match terminal.clear() {
        Err(error) => panic!("Error: {error:?}"),
        Ok(_) => (),
    };
    let app_result = run(terminal);
    ratatui::restore();
    return app_result;
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        match terminal.draw(|frame| {
            widgets::add_widgets_to_frame(frame);
        }) {
            Err(error) => panic!("Error: {error:?}"),
            Ok(_) => (),
        };
        if poll(Duration::from_millis(1000))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
