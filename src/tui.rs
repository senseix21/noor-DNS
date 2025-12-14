use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};
use std::io;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum DnsAction {
    Allow,
    Block,
    DnsOnly,
    NoMatch,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub client_ip: String,
    pub domain: String,
    pub action: DnsAction,
}

pub async fn run_tui(logs: Vec<LogEntry>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = logs
                .iter()
                .map(|entry| {
                    let color = match entry.action {
                        DnsAction::Allow => Color::Green,
                        DnsAction::Block => Color::Red,
                        DnsAction::DnsOnly => Color::Blue,
                        DnsAction::NoMatch => Color::Gray,
                    };
                    ListItem::new(format!(
                        "[{}] {} → {}",
                        entry.timestamp, entry.client_ip, entry.domain
                    ))
                    .style(Style::default().fg(color))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("noorDNS Logs").borders(Borders::ALL));
            f.render_widget(list, chunks[0]);
        })?;

        if event::poll(tick_rate - last_tick.elapsed())? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
