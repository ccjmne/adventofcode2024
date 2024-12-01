use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use registry::get;
use std::{fs, io, path::Path};

mod registry;
mod y2024 {
    pub mod d1;
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = (App {
        day: 1,
        year: 2024,
        exit: false,
    })
    .run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    day: u8,
    year: u16,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('k') => self.decrement_day(),
            KeyCode::Char('j') => self.increment_day(),
            KeyCode::Char('h') => self.decrement_year(),
            KeyCode::Char('l') => self.increment_year(),
            _ => {}
        }
    }

    fn increment_year(&mut self) {
        self.year += 1
    }
    fn decrement_year(&mut self) {
        self.year -= 1
    }

    fn increment_day(&mut self) {
        self.day = if self.day == 25 {
            self.increment_year();
            1
        } else {
            self.day + 1
        }
    }
    fn decrement_day(&mut self) {
        self.day = if self.day == 1 {
            self.decrement_year();
            25
        } else {
            self.day - 1
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

pub fn read_input(year: u16, day: u8, real: bool) -> String {
    fs::read_to_string(Path::new(file!()).parent().unwrap().join(format!(
        "y{year}/d{day}_{}.txt",
        if real { "real" } else { "test" }
    )))
    .unwrap_or(String::from(""))
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " [".into(),
            "j".light_blue().bold(),
            "|".into(),
            "k".light_blue().bold(),
            "]: navigate days,".into(),
            " [".into(),
            "h".light_blue().bold(),
            "|".into(),
            "l".light_blue().bold(),
            "]: navigate years,".into(),
            " [".into(),
            "q".light_blue().bold(),
            "]: quit ".into(),
        ]);
        let title = Line::from(vec![
            " AoC ".into(),
            self.year.to_string().bold(),
            " day ".into(),
            self.day.to_string().bold(),
            " ".into(),
        ]);
        let input = read_input(2024, self.day, false);
        let (i, ii) = match get(self.day) {
            Some(v) => (v.solve)(input),
            None => (
                Box::new("TODO".to_string()) as Box<dyn std::fmt::Display + Send + Sync>,
                Box::new("TODO".to_string()) as Box<dyn std::fmt::Display + Send + Sync>,
            ),
        };

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.right_aligned())
            .border_set(border::PLAIN);
        Paragraph::new(format!("Part I: {}\nPart II: {}", i, ii))
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let area = Rect::new(0, 0, 80, 4);
        let mut buf = Buffer::empty(area);

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┌──────────────────────────────── AoC 0 day 0 ─────────────────────────────────┐",
            "│                                                                              │",
            "│                                                                              │",
            "└────────────────────── [j|k]: navigate days, [h|l]: navigate years, [q]: quit ┘",
        ]);
        buf.set_style(area, Style::reset());
        expected.set_style(area, Style::reset());

        assert_eq!(buf, expected);
    }
}
