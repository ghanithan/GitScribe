# File Discovery Engine

The file discovery engine is responsible for finding and monitoring content files in the GitScribe project.

## Features

- Recursive Markdown file discovery
- Asset file detection and handling
- File change watching system
- Ignore patterns support (.gitignore style)

## Usage

### Scanning for Files

```rust
use gitscribe::content::{ContentScanner, ScanOptions, FileType};
use std::path::PathBuf;

// Create scanner with custom options
let options = ScanOptions {
    root_dir: PathBuf::from("./my-content"),
    include_patterns: vec!["**/*.md".to_string()],
    exclude_patterns: vec!["drafts/**".to_string()],
    follow_links: false,
};

let scanner = ContentScanner::new(options)?;

// Scan for files
let results = scanner.scan()?;

// Process found files
for result in results {
    match result.file_type {
        FileType::Markdown => {
            println!("Found Markdown file: {:?}", result.path);
        },
        FileType::Asset => {
            println!("Found asset file: {:?}", result.path);
        },
        FileType::Other => {
            println!("Found other file: {:?}", result.path);
        },
    }
}
```

### Watching for File Changes

```rust
use gitscribe::content::{ContentScanner, ScanOptions, FileEvent};
use tokio::runtime::Runtime;

// Create scanner with options
let scanner = ContentScanner::new(options)?;

// Create file watcher
let mut watcher = scanner.watch()?;

// Process file events asynchronously
let mut receiver = watcher.receiver();
tokio::spawn(async move {
    while let Some(event) = receiver.recv().await {
        match event {
            FileEvent::Created(path, file_type) => {
                println!("File created: {:?}", path);
            },
            FileEvent::Modified(path, file_type) => {
                println!("File modified: {:?}", path);
            },
            FileEvent::Deleted(path, file_type) => {
                println!("File deleted: {:?}", path);
            },
            FileEvent::Error(error) => {
                println!("Error: {}", error);
            },
        }
    }
});
```

## How It Works

The file discovery engine uses the following Rust crates:

- `ignore`: For file discovery with `.gitignore` pattern support
- `notify`: For file system change notifications
- `globset`: For glob pattern matching

### File Types

The engine distinguishes between three types of files:

- **Markdown**: Files with `.md` extension
- **Asset**: Files with extensions like `.jpg`, `.png`, `.svg`, `.css`, `.js`, etc.
- **Other**: Any other file type

### Performance Considerations

- The engine is designed to handle 1000+ files efficiently
- File watching is done in a separate thread to avoid blocking the main thread
- Events are sent through a channel with a buffer to avoid missing events during high activity periods