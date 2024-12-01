use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    widgets::Block,
    Terminal,
};
use std::io::{self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut one = true;
    let mut two = true;
    loop {
        terminal.draw(|frame| {
            use Constraint::{Fill, Length, Min};

            let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
            let [title_area, main_area, status_area] = vertical.areas(frame.area());

            if one {
                let horizontal = Layout::horizontal([Fill(1); 2]);
                let areas = horizontal.split(main_area);
                frame.render_widget(Block::bordered().title(" Part I "), areas[0]);
                frame.render_widget(Block::bordered().title(" Part II "), areas[1]);
            } else {
                let horizontal = Layout::horizontal([Fill(1)]);
                let [right_area] = horizontal.areas(main_area);
                frame.render_widget(Block::bordered().title(" Part II "), right_area);
            }

            frame.render_widget(Block::default().title("Advent of Code 2024"), title_area);
            frame.render_widget(Block::default().title("Status Bar"), status_area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('1') {
                one = !one;
            }

            if key.code == KeyCode::Char('2') {
                two = !two;
            }

            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
