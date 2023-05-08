use std::io::{Stdout, Write};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

/// enters raw mode
pub fn enter_ui() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// exits raw mode
pub fn exit_ui() -> anyhow::Result<()> {
    disable_raw_mode();
    let mut stdout = std::io::stdout();
    execute!(
        stdout, 
        LeaveAlternateScreen,
        EnableMouseCapture
    );
    std::io::stdout().write(b"\x1B[0 q")?;
    Ok(())
}
