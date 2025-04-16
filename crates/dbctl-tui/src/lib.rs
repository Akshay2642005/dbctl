use ratatui::{
    Terminal,
    backend::TermionBackend,
    widgets::{Block, Borders, List, ListItem},
};
use std::io;
use termion::raw::IntoRawMode;

pub fn run_tui() -> io::Result<()> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let items = vec![
            ListItem::new("PostgreSQL"),
            ListItem::new("Redis"),
            ListItem::new("MariaDB"),
        ];
        let list =
            List::new(items).block(Block::default().borders(Borders::ALL).title("Databases"));

        f.render_widget(list, size);
    })?;

    Ok(())
}
