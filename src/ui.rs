use colored::*;
use std::io::Write;

pub struct ProgressBar {
    current: usize,
    total: usize,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            current: 0,
            total,
            width: 40,
        }
    }

    pub fn update(&mut self, current: usize) {
        self.current = current;
        self.render();
    }

    fn render(&self) {
        let percent = (self.current as f32 / self.total as f32) * 100.0;
        let filled = (self.current as f32 / self.total as f32 * self.width as f32) as usize;

        print!("\r  [");
        print!("{}", "=".repeat(filled).cyan().bold());
        print!("{}", " ".repeat(self.width - filled));
        print!("] {}%", percent as i32);
        std::io::stdout().flush().unwrap();
    }

    pub fn finish(&self) {
        println!();
    }
}

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    col_widths: Vec<usize>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
        let col_widths = headers.iter().map(|h| h.len().max(15)).collect();
        
        Self {
            headers,
            rows: Vec::new(),
            col_widths,
        }
    }

    pub fn add_row(&mut self, cells: Vec<&str>) {
        let cells: Vec<String> = cells.iter().map(|c| c.to_string()).collect();
        for (i, cell) in cells.iter().enumerate() {
            if i < self.col_widths.len() {
                self.col_widths[i] = self.col_widths[i].max(cell.len());
            }
        }
        self.rows.push(cells);
    }

    pub fn print(&self) {
        println!();
        
        // Header
        print!("  ");
        for (i, header) in self.headers.iter().enumerate() {
            let width = self.col_widths.get(i).unwrap_or(&20);
            print!("{:<width$}", header.bold(), width = width + 2);
        }
        println!();

        // Separator
        print!("  ");
        for width in &self.col_widths {
            print!("{}", "-".repeat(width + 2));
        }
        println!();

        // Rows
        for row in &self.rows {
            print!("  ");
            for (i, cell) in row.iter().enumerate() {
                let width = self.col_widths.get(i).unwrap_or(&20);
                print!("{:<width$}", cell, width = width + 2);
            }
            println!();
        }
        println!();
    }
}

pub fn section(title: &str) {
    println!();
    println!("{}", title.bold().cyan());
    println!("{}", "-".repeat(title.len()).cyan());
}

pub fn subsection(title: &str) {
    println!();
    println!("  {}", title.bold());
    println!("  {}", "-".repeat(title.len()));
}

pub fn header(app_name: &str, version: &str) {
    println!();
    println!("{} {}", app_name.bold().cyan(), format!("v{}", version).bright_black());
    println!("{}", "=".repeat(60).cyan());
    println!();
}

pub fn success(msg: &str) {
    println!("  [+] {}", msg.green());
}

pub fn error(msg: &str) {
    println!("  [-] {}", msg.red());
}

pub fn warning(msg: &str) {
    println!("  [!] {}", msg.yellow());
}

pub fn info(msg: &str) {
    println!("  [*] {}", msg.bright_blue());
}

pub fn verbose(msg: &str) {
    println!("  [>] {}", msg.bright_black());
}

pub fn pair(key: &str, value: &str) {
    println!("  {:<25} {}", format!("{}:", key).bright_black(), value);
}

pub fn pairs(items: &[(&str, &str)]) {
    for (key, value) in items {
        pair(key, value);
    }
}

pub fn list_items(items: &[(&str, &str)]) {
    for (item, desc) in items {
        println!("  {:<20} {}", item.bright_white(), desc.bright_black());
    }
}

pub fn banner(text: &str, char: &str) {
    let width = text.len() + 4;
    println!();
    println!("  {}", char.repeat(width).cyan());
    println!("  {} {} {}", char.cyan(), text.bold().yellow(), char.cyan());
    println!("  {}", char.repeat(width).cyan());
    println!();
}

pub fn rule() {
    println!("  {}", "-".repeat(70).bright_black());
}

pub fn status_line(status: &str, message: &str) {
    let symbol = match status {
        "ok" => format!("[+]").green(),
        "err" => format!("[-]").red(),
        "warn" => format!("[!]").yellow(),
        "info" => format!("[*]").bright_blue(),
        _ => format!("[?]").bright_black(),
    };
    println!("  {} {}", symbol, message);
}

pub struct Spinner {
    frames: Vec<&'static str>,
    current: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["|", "/", "-", "\\"],
            current: 0,
        }
    }

    pub fn next_frame(&mut self) -> &'static str {
        let frame = self.frames[self.current % self.frames.len()];
        self.current += 1;
        frame
    }
}

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

pub fn format_duration(ms: u128) -> String {
    if ms < 1000 {
        format!("{} ms", ms)
    } else if ms < 60000 {
        format!("{:.2} s", ms as f64 / 1000.0)
    } else {
        let secs = ms / 1000;
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{} m {} s", mins, secs)
    }
}
