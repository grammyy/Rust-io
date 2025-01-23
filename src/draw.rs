use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    cpu_usage: &[String],
    memory_info: &str,
    disk_info: &[String],
    disk_processes: &[String],
    network_info: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let cpu_height = ((cpu_usage.len() as f64 / 5.0).ceil() + 2.0) as u16;

    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(cpu_height), //cpu
                Constraint::Length(3), // mem
                Constraint::Min(10),   // disk
                Constraint::Length(3), // networks
            ].as_ref())
            .split(frame.size());

        let disk_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(40), // disk usage
                    Constraint::Percentage(60), // disk processes
                ].as_ref(),
            )
            .split(chunks[2]);

        // ↓ chunks cpu cores into 5 item sections ↓
        let cpu_cores = cpu_usage
            .chunks(5)
            .map(|chunk| chunk.join(" ")) 
            .collect::<Vec<String>>()
            .join("\n");
    
        // ↓ cpu usage ↓
        frame.render_widget(
            Paragraph::new(
                cpu_cores
            ).block(Block::default().title("CPU Usage").borders(Borders::ALL)
        ), chunks[0]);

        // ↓ memory usage ↓
        frame.render_widget(
            Paragraph::new(
                memory_info
            ).block(Block::default().title("Memory Usage").borders(Borders::ALL)
        ), chunks[1]);

        // ↓ disk usage ↓
        frame.render_widget(
            Paragraph::new(
                disk_info.join("\n")
            ).block(Block::default().title("Disk Usage").borders(Borders::ALL)
        ), disk_chunks[0]);

        // ↓ disk processes ↓
        frame.render_widget(
            Paragraph::new(
                disk_processes.join("\n")
            ).block(Block::default().title("Disk Processes").borders(Borders::ALL)
        ), disk_chunks[1]);

        // ↓ network info ↓
        frame.render_widget(
            Paragraph::new(
                network_info.join("\n")
            ).block(Block::default().title("Network Activity").borders(Borders::ALL)
        ), chunks[3]);
    })?;
    Ok(())
}