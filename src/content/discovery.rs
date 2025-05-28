use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use ignore::Walk;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use globset::{Glob, GlobSetBuilder};

use tokio::sync::mpsc::{self, Receiver};

/// Types of files supported by the file discovery engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Markdown files (.md)
    Markdown,
    /// Asset files (images, stylesheets, etc.)
    Asset,
    /// Other files
    Other,
}

/// Result of a file scan
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Path to the file
    pub path: PathBuf,
    /// Type of the file
    pub file_type: FileType,
    /// Last modified time of the file
    pub last_modified: SystemTime,
}

/// Options for file scanning
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Root directory to scan
    pub root_dir: PathBuf,
    /// List of glob patterns to include
    pub include_patterns: Vec<String>,
    /// List of glob patterns to exclude (in addition to .gitignore)
    pub exclude_patterns: Vec<String>,
    /// Whether to follow symlinks
    pub follow_links: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("."),
            include_patterns: vec![
                "**/*.md".to_string(),
                "**/*.jpg".to_string(), 
                "**/*.jpeg".to_string(),
                "**/*.png".to_string(),
                "**/*.gif".to_string(),
                "**/*.svg".to_string(),
                "**/*.css".to_string(),
                "**/*.js".to_string(),
            ],
            exclude_patterns: vec![],
            follow_links: false,
        }
    }
}

/// File event types for the watcher
#[derive(Debug, Clone)]
pub enum FileEvent {
    /// File was created
    Created(PathBuf, FileType),
    /// File was modified
    Modified(PathBuf, FileType),
    /// File was deleted
    Deleted(PathBuf, FileType),
    /// Error occurred
    Error(String),
}

/// Content scanner for discovering and watching files
pub struct ContentScanner {
    options: ScanOptions,
    include_glob_set: globset::GlobSet,
    exclude_glob_set: globset::GlobSet,
}

impl ContentScanner {
    /// Create a new content scanner with the given options
    pub fn new(options: ScanOptions) -> Result<Self, String> {
        // Compile glob patterns
        let mut include_builder = GlobSetBuilder::new();
        for pattern in &options.include_patterns {
            let glob = Glob::new(pattern)
                .map_err(|e| format!("Invalid include pattern '{}': {}", pattern, e))?;
            include_builder.add(glob);
        }
        
        let mut exclude_builder = GlobSetBuilder::new();
        for pattern in &options.exclude_patterns {
            let glob = Glob::new(pattern)
                .map_err(|e| format!("Invalid exclude pattern '{}': {}", pattern, e))?;
            exclude_builder.add(glob);
        }
        
        let include_glob_set = include_builder.build()
            .map_err(|e| format!("Failed to build include glob set: {}", e))?;
        
        let exclude_glob_set = exclude_builder.build()
            .map_err(|e| format!("Failed to build exclude glob set: {}", e))?;
        
        Ok(Self {
            options,
            include_glob_set,
            exclude_glob_set,
        })
    }
    
    /// Scan for files in the specified directory
    pub fn scan(&self) -> Result<Vec<ScanResult>, String> {
        let mut results = Vec::new();
        
        // Use the ignore crate to walk the directory tree
        // This automatically respects .gitignore files
        let walker = Walk::new(&self.options.root_dir);
            
        for entry in walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    log::warn!("Error walking directory: {}", err);
                    continue;
                }
            };
            
            let path = entry.path();
            
            // Skip directories
            if path.is_dir() {
                continue;
            }
            
            // Check if the file matches our patterns
            let rel_path = match path.strip_prefix(&self.options.root_dir) {
                Ok(rel_path) => rel_path,
                Err(_) => {
                    log::warn!("Failed to strip prefix from path: {:?}", path);
                    continue;
                }
            };
            
            // Skip if the file does not match include patterns
            if !self.include_glob_set.is_match(rel_path) {
                continue;
            }
            
            // Skip if the file matches exclude patterns
            if self.exclude_glob_set.is_match(rel_path) {
                continue;
            }
            
            // Determine file type
            let file_type = self.determine_file_type(path);
            
            // Get last modified time
            let metadata = match std::fs::metadata(path) {
                Ok(metadata) => metadata,
                Err(err) => {
                    log::warn!("Failed to get metadata for file {:?}: {}", path, err);
                    continue;
                }
            };
            
            let last_modified = metadata.modified().unwrap_or(SystemTime::now());
            
            results.push(ScanResult {
                path: path.to_path_buf(),
                file_type,
                last_modified,
            });
        }
        
        Ok(results)
    }
    
    /// Determine the type of a file based on its extension
    fn determine_file_type(&self, path: &Path) -> FileType {
        if let Some(extension) = path.extension() {
            if extension.eq_ignore_ascii_case("md") {
                return FileType::Markdown;
            } else if extension.eq_ignore_ascii_case("jpg") 
                || extension.eq_ignore_ascii_case("jpeg")
                || extension.eq_ignore_ascii_case("png")
                || extension.eq_ignore_ascii_case("gif")
                || extension.eq_ignore_ascii_case("svg")
                || extension.eq_ignore_ascii_case("css")
                || extension.eq_ignore_ascii_case("js") {
                return FileType::Asset;
            }
        }
        
        FileType::Other
    }
    
    /// Create a file watcher that will send file events to the returned channel
    pub fn watch(&self) -> Result<ContentWatcher, String> {
        let (tx, rx) = mpsc::channel(100);
        
        let file_types = Arc::new(Mutex::new(HashMap::new()));
        
        // Scan for initial files and store their types
        let scan_results = self.scan()?;
        for result in scan_results {
            file_types.lock().unwrap().insert(result.path, result.file_type);
        }
        
        // Create a watcher
        let config = Config::default()
            .with_poll_interval(std::time::Duration::from_secs(1));
        
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        if let Some(path) = event.paths.first() {
                            // Skip directories
                            if path.is_dir() {
                                return;
                            }
                            
                            let mut file_types = file_types.lock().unwrap();
                            
                            match event.kind {
                                EventKind::Create(_) => {
                                    // Determine file type for new file
                                    let file_type = Self::determine_file_type_static(path);
                                    file_types.insert(path.clone(), file_type);
                                    let _ = tx.try_send(FileEvent::Created(path.clone(), file_type));
                                },
                                EventKind::Modify(_) => {
                                    // Get file type from cache or determine if not found
                                    let file_type = match file_types.get(path) {
                                        Some(file_type) => *file_type,
                                        None => {
                                            let file_type = Self::determine_file_type_static(path);
                                            file_types.insert(path.clone(), file_type);
                                            file_type
                                        }
                                    };
                                    let _ = tx.try_send(FileEvent::Modified(path.clone(), file_type));
                                },
                                EventKind::Remove(_) => {
                                    // Get file type from cache or use Other if not found
                                    let file_type = file_types.remove(path).unwrap_or(FileType::Other);
                                    let _ = tx.try_send(FileEvent::Deleted(path.clone(), file_type));
                                },
                                _ => { /* Ignore other events */ }
                            }
                        }
                    },
                    Err(e) => {
                        let _ = tx.try_send(FileEvent::Error(format!("Watch error: {}", e)));
                    }
                }
            },
            config,
        ).map_err(|e| format!("Failed to create watcher: {}", e))?;
        
        // Start watching the root directory
        watcher.watch(
            Path::new(&self.options.root_dir), 
            RecursiveMode::Recursive
        ).map_err(|e| format!("Failed to watch directory: {}", e))?;
        
        Ok(ContentWatcher {
            _watcher: watcher,
            receiver: rx,
        })
    }
    
    /// Static version of determine_file_type for use in the watcher callback
    fn determine_file_type_static(path: &Path) -> FileType {
        if let Some(extension) = path.extension() {
            if extension.eq_ignore_ascii_case("md") {
                return FileType::Markdown;
            } else if extension.eq_ignore_ascii_case("jpg") 
                || extension.eq_ignore_ascii_case("jpeg")
                || extension.eq_ignore_ascii_case("png")
                || extension.eq_ignore_ascii_case("gif")
                || extension.eq_ignore_ascii_case("svg")
                || extension.eq_ignore_ascii_case("css")
                || extension.eq_ignore_ascii_case("js") {
                return FileType::Asset;
            }
        }
        
        FileType::Other
    }
}

/// Content watcher for watching file changes
pub struct ContentWatcher {
    // Need to keep the watcher alive for the channel to work
    _watcher: RecommendedWatcher,
    receiver: Receiver<FileEvent>,
}

impl ContentWatcher {
    /// Get the receiver for file events
    pub fn receiver(&mut self) -> &mut Receiver<FileEvent> {
        &mut self.receiver
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;
    
    #[test]
    fn test_scan_markdown_files() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test files
        let md_file_path = temp_path.join("test.md");
        let _md_file = File::create(&md_file_path).unwrap();
        
        let txt_file_path = temp_path.join("test.txt");
        let _txt_file = File::create(&txt_file_path).unwrap();
        
        // Create a scanner with default options
        let options = ScanOptions {
            root_dir: temp_path.to_path_buf(),
            ..Default::default()
        };
        
        let scanner = ContentScanner::new(options).unwrap();
        let results = scanner.scan().unwrap();
        
        // Should find only the markdown file
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, md_file_path);
        assert_eq!(results[0].file_type, FileType::Markdown);
    }
    
    #[test]
    fn test_scan_asset_files() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test files
        let jpg_file_path = temp_path.join("test.jpg");
        let _jpg_file = File::create(&jpg_file_path).unwrap();
        
        let png_file_path = temp_path.join("test.png");
        let _png_file = File::create(&png_file_path).unwrap();
        
        let css_file_path = temp_path.join("test.css");
        let _css_file = File::create(&css_file_path).unwrap();
        
        // Create a scanner with default options
        let options = ScanOptions {
            root_dir: temp_path.to_path_buf(),
            ..Default::default()
        };
        
        let scanner = ContentScanner::new(options).unwrap();
        let results = scanner.scan().unwrap();
        
        // Should find all asset files
        assert_eq!(results.len(), 3);
        
        // Verify file types
        let file_types: Vec<FileType> = results.iter().map(|r| r.file_type).collect();
        assert!(file_types.contains(&FileType::Asset));
        assert_eq!(file_types.iter().filter(|&&t| t == FileType::Asset).count(), 3);
    }
    
    #[test]
    fn test_exclude_patterns() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test files
        let md_file_path = temp_path.join("test.md");
        let _md_file = File::create(&md_file_path).unwrap();
        
        let ignored_file_path = temp_path.join("ignored.md");
        let _ignored_file = File::create(&ignored_file_path).unwrap();
        
        // Create a scanner with explicit exclude patterns
        let options = ScanOptions {
            root_dir: temp_path.to_path_buf(),
            exclude_patterns: vec!["**/ignored.md".to_string()],
            ..Default::default()
        };
        
        let scanner = ContentScanner::new(options).unwrap();
        let results = scanner.scan().unwrap();
        
        // Count files by name
        let md_files = results.iter()
            .filter(|r| r.path.file_name().unwrap_or_default() == "test.md")
            .count();
        
        let ignored_files = results.iter()
            .filter(|r| r.path.file_name().unwrap_or_default() == "ignored.md")
            .count();
            
        // Should find the non-ignored markdown file and not the ignored one
        assert_eq!(md_files, 1);
        assert_eq!(ignored_files, 0);
    }
}