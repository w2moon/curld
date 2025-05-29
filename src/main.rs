use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local directory path
    #[arg(short = 'd', required = true)]
    directory: PathBuf,

    /// Target URL for upload
    #[arg(required = true)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Verify if directory exists
    if !args.directory.exists() {
        anyhow::bail!("Directory does not exist: {:?}", args.directory);
    }

    // Traverse all files in the directory
    for entry in WalkDir::new(&args.directory) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let relative_path = file_path.strip_prefix(&args.directory)?;
            
            // Build complete URL
            let url = format!("{}/{}", args.url.trim_end_matches('/'), relative_path.to_string_lossy());
            
            // Read file content
            let mut file = File::open(file_path).await?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await?;
            
            // Send PUT request with file content
            let client = reqwest::Client::new();
            let response = client
                .put(&url)
                .body(contents)
                .send()
                .await
                .context("Failed to upload file")?;
            
            if response.status().is_success() {
                println!("Successfully uploaded: {}", relative_path.display());
            } else {
                eprintln!("Upload failed {}: {}", relative_path.display(), response.status());
            }
        }
    }

    Ok(())
}
