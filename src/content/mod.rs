// Content processing module
// Handles file discovery, content parsing, and content transformation

mod discovery;
mod demo;

pub use discovery::{ContentScanner, FileType, ScanResult, ScanOptions, FileEvent, ContentWatcher};
pub use demo::{discovery_demo, run_discovery_demo};