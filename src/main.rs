use std::io;

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
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.right_aligned())
            .border_set(border::PLAIN);
        Paragraph::new("").centered().block(block).render(area, buf);
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
