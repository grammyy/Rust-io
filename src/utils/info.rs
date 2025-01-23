use sysinfo::{CpuExt, DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn collect(system: &mut System) -> (Vec<String>, String, Vec<String>, Vec<String>, Vec<String>) {
    system.refresh_all();
    system.refresh_cpu();

    // ↓ gather all cpu cores ↓
    let cpu_usage = system
        .cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| format!("Core {}: {:.2}%", i, cpu.cpu_usage()))
        .collect::<Vec<String>>();

    let memory_info = format!(
        "Memory: {} MB / {} MB",
        system.used_memory() / 1024,
        system.total_memory() / 1024
    );

    let disk_info = system
        .disks()
        .iter()
        .map(|disk| {
            format!(
                "{}: {} MB / {} MB",
                disk.name().to_string_lossy(),
                disk.available_space() / 1_048_576,
                disk.total_space() / 1_048_576
            )
        })
        .collect::<Vec<String>>();

    let disk_processes = system
        .processes()
        .iter()
        .filter_map(|(_, process)| {
            let disk_usage = process.disk_usage();
            if disk_usage.total_written_bytes > 0 || disk_usage.total_read_bytes > 0 {
                Some(format!(
                    "{}: Read {} bytes, Wrote {} bytes",
                    process.name(),
                    disk_usage.total_read_bytes,
                    disk_usage.total_written_bytes
                ))
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let network_info = system
        .networks()
        .iter()
        .map(|(iface, data)| {
            format!(
                "{}: Received {} bytes, Transmitted {} bytes",
                iface,
                data.received(),
                data.transmitted()
            )
        })
        .collect::<Vec<String>>();

    (
        cpu_usage,
        memory_info,
        disk_info,
        disk_processes,
        network_info,
    )
}