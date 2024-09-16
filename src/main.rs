use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use pulldown_cmark::{html, Parser as MdParser};
use tempfile::Builder;

/// Simple CLI app to render Markdown files in a browser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Markdown file to render
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Read Markdown content
    let (file_name, markdown_input) = match &args.file {
        Some(file_path) => {
            let mut file = File::open(&file_path).unwrap_or_else(|err| {
                eprintln!("Error opening file {}: {}", file_path.display(), err);
                std::process::exit(1);
            });
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            (file_path.file_name().unwrap().to_string_lossy().to_string(), content)
        }
        None => {
            // Read from stdin if no file is provided
            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;
            (String::from("New file"), content)
        }
    };

    // Convert Markdown to HTML
    let parser = MdParser::new(&markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Create full HTML document
    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; max-width: 800px; margin: 0 auto; }}
        pre {{ background-color: #f4f4f4; padding: 10px; border-radius: 5px; }}
        code {{ background-color: #f4f4f4; padding: 2px 4px; border-radius: 3px; }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
        file_name, html_output
    );

    // Create a temporary file with .html extension
    let temp_file = Builder::new()
        .prefix("markdown_preview_")
        .suffix(".html")
        .rand_bytes(5)
        .tempfile()?;
    let temp_path = temp_file.path().to_string_lossy().to_string();

    // Write HTML content to the temporary file
    let mut file = temp_file.as_file();
    file.write_all(html_content.as_bytes())?;
    file.flush()?;

    // Open the default browser
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&temp_path)
            .spawn()
            .expect("Failed to open browser");
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", &temp_path])
            .spawn()
            .expect("Failed to open browser");
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&temp_path)
            .spawn()
            .expect("Failed to open browser");
    }

    // Keep the program running to prevent the temporary file from being deleted
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
