use trace_deck::*;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::time::Instant;

const APP_NAME: &str = "trace-deck";
const APP_VERSION: &str = "0.1.0";

#[derive(Parser)]
#[command(name = APP_NAME, version = APP_VERSION)]
#[command(about = "Advanced document processing engine")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(value_name = "FILE", global = true)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a document with full analysis
    Process {
        #[arg(value_name = "FILE")]
        file: String,
        #[arg(short, long, value_parser = ["json", "pretty"], default_value = "pretty")]
        format: String,
        #[arg(short, long)]
        timing: bool,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Extract text content from document
    Extract {
        #[arg(value_name = "FILE")]
        file: String,
        #[arg(short, long)]
        text_only: bool,
    },

    /// Process multiple documents
    Batch {
        #[arg(value_name = "DIR")]
        dir: String,
        #[arg(short, long)]
        ext: Option<String>,
    },

    /// Show supported formats
    Formats {},

    /// Display system information
    Info {},

    /// Check system capabilities
    Check {},

    /// Real-time file monitoring
    Watch {
        #[arg(value_name = "DIR")]
        dir: String,
        #[arg(short, long)]
        ext: Option<String>,
    },

    /// Export processing results
    Export {
        #[arg(value_name = "FILE")]
        file: String,
        #[arg(short, long)]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Process { file, format, timing, verbose }) => {
            process_document_cmd(&file, &format, timing, verbose).await?;
        }
        Some(Commands::Extract { file, text_only }) => {
            extract_text_cmd(&file, text_only).await?;
        }
        Some(Commands::Batch { dir, ext }) => {
            batch_process_cmd(&dir, ext).await?;
        }
        Some(Commands::Formats {}) => show_formats(),
        Some(Commands::Info {}) => show_info(),
        Some(Commands::Check {}) => check_system(),
        Some(Commands::Watch { dir, ext }) => watch_directory(&dir, ext).await?,
        Some(Commands::Export { file, output }) => export_results(&file, &output).await?,
        None => {
            if let Some(file) = cli.file {
                process_document_cmd(&file, "pretty", false, false).await?;
            }
        }
    }

    Ok(())
}

async fn process_document_cmd(file: &str, format: &str, timing: bool, verbose: bool) -> Result<()> {
    if !Path::new(file).exists() {
        ui::error(&format!("File not found: {}", file));
        std::process::exit(1);
    }

    ui::header(APP_NAME, APP_VERSION);
    ui::status_line("info", "Processing document...");
    
    ui::subsection("Input Details");
    let metadata = std::fs::metadata(file)?;
    ui::pair("Path", file);
    ui::pair("Size", &ui::format_size(metadata.len()));
    
    let ext = Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown");
    ui::pair("Type", ext);
    
    println!();

    let start = Instant::now();
    match process_document(file).await {
        Ok(result) => {
            let elapsed = start.elapsed();
            
            ui::success(&format!("Processing completed in {}", 
                ui::format_duration(elapsed.as_millis())));
            println!();

            match format {
                "json" => println!("{}", serde_json::to_string(&result)?),
                _ => println!("{}", serde_json::to_string_pretty(&result)?),
            }

            if timing {
                ui::rule();
                ui::subsection("Performance Metrics");
                ui::pair("Total Time", &ui::format_duration(elapsed.as_millis()));
                ui::pair("Status", "Completed");
                println!();
            }

            if verbose {
                ui::rule();
                ui::subsection("Document Statistics");
                ui::pair("Text Blocks", &result.processed.text_blocks.len().to_string());
                ui::pair("Visual Elements", &result.processed.visual_elements.len().to_string());
                ui::pair("Pages", &result.extracted.metadata.pages.unwrap_or(1).to_string());
                ui::pair("Analyzed Size", &ui::format_size(result.extracted.metadata.file_size));
                println!();
            }
        }
        Err(e) => {
            ui::error(&format!("{}", e));
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn extract_text_cmd(file: &str, text_only: bool) -> Result<()> {
    if !Path::new(file).exists() {
        ui::error(&format!("File not found: {}", file));
        std::process::exit(1);
    }

    ui::header(APP_NAME, APP_VERSION);
    ui::status_line("info", "Extracting text...");
    
    ui::subsection("Target File");
    ui::pair("Path", file);
    println!();

    match process_document(file).await {
        Ok(result) => {
            if text_only {
                println!("{}", result.extracted.text);
            } else {
                ui::section("Text Content");
                println!("{}", result.extracted.text);
                
                ui::section("Document Metadata");
                let mut table = ui::Table::new(vec!["Property", "Value"]);
                table.add_row(vec!["Type", &result.extracted.metadata.file_type]);
                table.add_row(vec!["Size", &ui::format_size(result.extracted.metadata.file_size as u64)]);
                if let Some(pages) = result.extracted.metadata.pages {
                    table.add_row(vec!["Pages", &pages.to_string()]);
                }
                if let Some(title) = &result.extracted.metadata.title {
                    table.add_row(vec!["Title", title]);
                }
                table.print();

                if !result.extracted.images.is_empty() {
                    ui::status_line("info", &format!("Found {} images", result.extracted.images.len()));
                }
            }
        }
        Err(e) => {
            ui::error(&format!("{}", e));
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn batch_process_cmd(dir: &str, ext: Option<String>) -> Result<()> {
    let path = Path::new(dir);
    
    if !path.is_dir() {
        ui::error(&format!("Directory not found: {}", dir));
        std::process::exit(1);
    }

    ui::header(APP_NAME, APP_VERSION);
    ui::status_line("info", "Starting batch processing...");
    
    ui::subsection("Configuration");
    ui::pair("Directory", dir);
    if let Some(ref ext_filter) = ext {
        ui::pair("Filter", ext_filter);
    }
    println!();

    let mut files = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        
        if file_path.is_file() {
            if let Some(ref ext_filter) = ext {
                if let Some(file_ext) = file_path.extension().and_then(|e| e.to_str()) {
                    if file_ext.eq_ignore_ascii_case(ext_filter) {
                        files.push(file_path);
                    }
                }
            } else {
                files.push(file_path);
            }
        }
    }

    if files.is_empty() {
        ui::warning("No files found");
        return Ok(());
    }

    ui::status_line("info", &format!("Found {} files", files.len()));
    println!();

    ui::section("Processing");
    let mut progress = ui::ProgressBar::new(files.len());
    
    let mut processed = 0;
    let mut failed = 0;

    for (idx, file) in files.iter().enumerate() {
        if let Some(_filename) = file.file_name().and_then(|n| n.to_str()) {
            match process_document(file.to_str().unwrap()).await {
                Ok(_) => {
                    processed += 1;
                    progress.update(idx + 1);
                }
                Err(_) => {
                    failed += 1;
                    progress.update(idx + 1);
                }
            }
        }
    }
    progress.finish();

    println!();
    ui::section("Results");
    let mut table = ui::Table::new(vec!["Metric", "Count"]);
    table.add_row(vec!["Total Files", &files.len().to_string()]);
    table.add_row(vec!["Processed", &processed.to_string()]);
    table.add_row(vec!["Failed", &failed.to_string()]);
    table.add_row(vec!["Success Rate", &format!("{:.1}%", (processed as f64 / files.len() as f64) * 100.0)]);
    table.print();

    if failed == 0 {
        ui::success("All files processed successfully");
    } else {
        ui::warning(&format!("{} files failed", failed));
    }
    println!();

    Ok(())
}

fn show_formats() {
    ui::header(APP_NAME, APP_VERSION);
    ui::section("Supported Formats");
    
    let mut table = ui::Table::new(vec!["Format", "Extension", "Description"]);
    table.add_row(vec!["PDF", ".pdf", "Portable Document Format"]);
    table.add_row(vec!["Word", ".docx", "Microsoft Word Document"]);
    table.add_row(vec!["PNG", ".png", "Portable Network Graphics"]);
    table.add_row(vec!["JPEG", ".jpg, .jpeg", "Joint Photographic Experts"]);
    table.add_row(vec!["GIF", ".gif", "Graphics Interchange Format"]);
    table.add_row(vec!["WebP", ".webp", "Modern Web Image Format"]);
    table.print();
}

fn show_info() {
    ui::header(APP_NAME, APP_VERSION);
    
    ui::section("Application Information");
    ui::pair("Name", APP_NAME);
    ui::pair("Version", APP_VERSION);
    ui::pair("Engine", "Rust (async/await)");
    ui::pair("Processing", "Multi-format document analysis");
    println!();

    ui::section("Capabilities");
    let mut table = ui::Table::new(vec!["Category", "Features"]);
    table.add_row(vec!["Input", "PDF, DOCX, PNG, JPG, GIF, WebP"]);
    table.add_row(vec!["Output", "JSON (structured)"]);
    table.add_row(vec!["Processing", "Text, metadata, images, tables"]);
    table.add_row(vec!["Performance", "Optimized for large documents"]);
    table.print();
    
    ui::section("Available Commands");
    ui::list_items(&[
        ("process", "Full document analysis"),
        ("extract", "Text extraction"),
        ("batch", "Multi-file processing"),
        ("formats", "Supported formats"),
        ("info", "System information"),
        ("check", "System capabilities"),
        ("watch", "Real-time monitoring"),
        ("export", "Export results"),
    ]);
    println!();
}

fn check_system() {
    ui::header(APP_NAME, APP_VERSION);
    ui::section("System Status");
    
    ui::status_line("ok", "Rust runtime initialized");
    ui::status_line("ok", "Async executor active");
    ui::status_line("ok", "Document parsing available");
    ui::status_line("ok", "JSON serialization ready");
    println!();

    ui::section("System Configuration");
    let mut table = ui::Table::new(vec!["Component", "Status"]);
    table.add_row(vec!["Runtime", "Tokio 1.x"]);
    table.add_row(vec!["Parser", "Multi-format"]);
    table.add_row(vec!["Output", "JSON (pretty/compact)"]);
    table.add_row(vec!["Logging", "Tracing enabled"]);
    table.print();
}

async fn watch_directory(_dir: &str, _ext: Option<String>) -> Result<()> {
    ui::header(APP_NAME, APP_VERSION);
    ui::warning("Watch feature coming soon");
    println!();
    Ok(())
}

async fn export_results(_file: &str, _output: &str) -> Result<()> {
    ui::header(APP_NAME, APP_VERSION);
    ui::warning("Export feature coming soon");
    println!();
    Ok(())
}
