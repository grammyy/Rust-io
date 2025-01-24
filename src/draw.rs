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
    terminal.draw(|frame| {
        let cpu_height = ((cpu_usage.len() as f64 / 5.0).ceil() + 2.0) as u16;
        let size = frame.size();

        let (main_direction, process_constraint, disk_constraint, network_constraint) = if size.width < size.height {
            // ↓ vertical layout, swap network and processes sections ↓
            (
                Direction::Vertical,
                Constraint::Percentage(40),
                Constraint::Length(10), // set disk to fit-content-like behavior
                Constraint::Length(5),  // set network to a small height
            )
        } else {
            // ↓ horizontal layout, processes on the right, network stays on the bottom ↓
            (
                Direction::Horizontal,
                Constraint::Percentage(40),
                Constraint::Min(10),  // keep disk with a minimum size
                Constraint::Length(5), // network size remains fixed
            )
        };

        let main_chunks = Layout::default()
            .direction(main_direction)
            .constraints([
                Constraint::Min(70), // main section
                process_constraint,  // processes section
            ])
            .split(size);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(cpu_height), // cpu cores
                Constraint::Length(3), // memory
                disk_constraint, // dynamically adjusted disk section
                network_constraint, // network section gets adjusted
            ])
            .split(main_chunks[0]);

        // ↓ cpu cores ↓
        let cpu_cores = cpu_usage
            .chunks(5)
            .map(|chunk| chunk.join(" "))
            .collect::<Vec<String>>()
            .join("\n");

        frame.render_widget(
            Paragraph::new(cpu_cores).block(Block::default().title("CPU Usage").borders(Borders::ALL)),
            left_chunks[0],
        );

        // ↓ memory usage ↓
        frame.render_widget(
            Paragraph::new(memory_info).block(Block::default().title("Memory Usage").borders(Borders::ALL)),
            left_chunks[1],
        );

        // ↓ disk usage ↓
        frame.render_widget(
            Paragraph::new(disk_info.join("\n")).block(Block::default().title("Disk Usage").borders(Borders::ALL)),
            left_chunks[2],
        );

        // ↓ network activity or disk processes, depending on orientation ↓
        if size.width < size.height {
            // If width < height, swap network and processes
            frame.render_widget(
                Paragraph::new(disk_processes.join("\n")).block(Block::default().title("Disk Processes").borders(Borders::ALL)),
                left_chunks[3],
            );
            frame.render_widget(
                Paragraph::new(network_info.join("\n")).block(Block::default().title("Network Activity").borders(Borders::ALL)),
                main_chunks[1],
            );
        } else {
            // Otherwise, keep original order
            frame.render_widget(
                Paragraph::new(network_info.join("\n")).block(Block::default().title("Network Activity").borders(Borders::ALL)),
                left_chunks[3],
            );
            frame.render_widget(
                Paragraph::new(disk_processes.join("\n")).block(Block::default().title("Disk Processes").borders(Borders::ALL)),
                main_chunks[1],
            );
        }
    })?;

    Ok(())
}