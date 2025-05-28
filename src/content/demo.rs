use std::path::PathBuf;
use tokio::runtime::Runtime;

use crate::content::{ContentScanner, FileEvent, FileType, ScanOptions};

/// Demo function to show how to use the file discovery engine
pub async fn discovery_demo(root_dir: PathBuf) -> Result<(), String> {
    // Create scanner with custom options
    let options = ScanOptions {
        root_dir: root_dir.clone(),
        include_patterns: vec![
            "**/*.md".to_string(),
            "**/*.jpg".to_string(),
            "**/*.png".to_string(),
        ],
        exclude_patterns: vec![
            "node_modules/**".to_string(),
            "target/**".to_string(),
        ],
        follow_links: false,
    };
    
    let scanner = ContentScanner::new(options)?;
    
    // Scan for files
    println!("Scanning for files in {:?}...", root_dir);
    
    let results = scanner.scan()?;
    
    println!("Found {} files:", results.len());
    
    // Count files by type
    let mut markdown_count = 0;
    let mut asset_count = 0;
    let mut other_count = 0;
    
    for result in &results {
        match result.file_type {
            FileType::Markdown => markdown_count += 1,
            FileType::Asset => asset_count += 1,
            FileType::Other => other_count += 1,
        }
    }
    
    println!("  Markdown files: {}", markdown_count);
    println!("  Asset files: {}", asset_count);
    println!("  Other files: {}", other_count);
    
    // Set up file watcher
    println!("Setting up file watcher...");
    
    let mut watcher = scanner.watch()?;
    
    println!("Watching for file changes (press Ctrl+C to stop):");
    
    // Process file events
    let mut receiver = watcher.receiver();
    
    // Wait for and process up to 10 events (just for demo purposes)
    for _ in 0..10 {
        match receiver.recv().await {
            Some(FileEvent::Created(path, file_type)) => {
                println!("File created: {:?} (type: {:?})", path, file_type);
            },
            Some(FileEvent::Modified(path, file_type)) => {
                println!("File modified: {:?} (type: {:?})", path, file_type);
            },
            Some(FileEvent::Deleted(path, file_type)) => {
                println!("File deleted: {:?} (type: {:?})", path, file_type);
            },
            Some(FileEvent::Error(error)) => {
                println!("Error: {}", error);
            },
            None => {
                println!("Watcher channel closed");
                break;
            },
        }
    }
    
    Ok(())
}

/// Synchronous version for non-async contexts
pub fn run_discovery_demo(root_dir: PathBuf) -> Result<(), String> {
    // Create a Tokio runtime for async operations
    let runtime = Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;
    
    // Run the async demo function to completion
    runtime.block_on(discovery_demo(root_dir))
}