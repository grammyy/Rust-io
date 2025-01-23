use std::{sync::Arc, time::Duration};
use sysinfo::{System, SystemExt};
use signal_hook::consts::SIGINT;
use signal_hook::flag as signal_flag;

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// ↓ modules ↓
#[path = "utils/info.rs"] mod info;
#[path = "utils/share.rs"] mod share;
mod draw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    // ↑ terminal state ↑

    signal_flag::register(SIGINT, running_clone)?;
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ↓ terminal loop ↓
    let res = run_app(&mut terminal, running);

    // ↓ restore after exit ↓
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    running: Arc<std::sync::atomic::AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut system = System::new_all();
    // ↑ system reference ↑

    while running.load(std::sync::atomic::Ordering::Relaxed) {
        let (cpu_usage, memory_info, disk_info, disk_processes, network_info) = info::collect(&mut system);

        draw::draw(
            terminal,
            &cpu_usage,
            &memory_info,
            &disk_info,
            &disk_processes,
            &network_info,
        )?;

        // ↓ input polling ↓
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // q -> exit
                }

                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break; // ctrl c -> exit
                }
            }
        }
    }

    Ok(())
}