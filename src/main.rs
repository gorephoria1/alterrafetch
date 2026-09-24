use sysinfo::System;

fn main() {
    // Subnautica/Alterra Theme Colors
    let cyan = "\x1b[36m";
    let white = "\x1b[37m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    // Initialize sysinfo and populate data
    let mut sys = System::new_all();
    sys.refresh_all(); 

    // 1. Get User and Hostname
    let user = std::env::var("USER").unwrap_or_else(|_| "Survivor".to_string());
    let host = System::host_name().unwrap_or_else(|| "Alterra-PDA".to_string());

    // 2. Get OS and Architecture
    let os_name = System::name().unwrap_or_else(|| "Subnautica-OS".to_string());
    let arch = std::env::consts::ARCH;

    // 3. Get CPU Model Name
    let cpu_name = sys.cpus()
        .first()
        .map(|cpu| cpu.brand().trim().to_string())
        .unwrap_or_else(|| "Alterra Bio-Processor".to_string());

    // 4. Calculate RAM usage in MB
    let total_mem = sys.total_memory() / 1_048_576; // Convert bytes to MB
    let used_mem = sys.used_memory() / 1_048_576;   

    // 5. Format Uptime as Survival Time (Days, Hours, Mins)
    let total_seconds = System::uptime();
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    
    let survival_time = if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    };

    // Print Layout: ASCII Logo on left, Live Specs on right
    println!("{}         /\\{}          {}{}@{}{}", cyan, reset, bold, user, host, reset);
    println!("{}        /  \\{}         {}", cyan, reset, "-----------------------------");
    println!("{}       /    \\{}        {}PDA Firmware:{} {} ({})", cyan, reset, cyan, reset, os_name, arch);
    println!("{}      /______\\{}       {}Survival Time:{} {}", cyan, reset, cyan, reset, survival_time);
    println!("{}                     {}Processor:{} {}{}", cyan, reset, cyan, reset, cpu_name);
    println!("{}         /\\{}          {}Biomass/RAM:{} {}MB / {}MB", white, reset, cyan, reset, used_mem, total_mem);
    println!("{}        /  \\{}         {}Survival State:{} Oxygen Stable", white, reset, cyan, reset);
    println!("{}       /____\\{}", white, reset);
    println!("{}      /      \\{}", white, reset);
    println!("{}     /________\\{}", white, reset);
    println!();
    println!("{}   A L T E R R A{}", cyan, reset);
}

