use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=tailwind.config.json");
    println!("cargo:rerun-if-changed=styles/input.css");
    
    // Get the project directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Check if we need to download Tailwind binaries
    let binaries_dir = Path::new(&manifest_dir).join(".tailwindcss-binaries");
    if !binaries_dir.exists() || binaries_dir.read_dir().unwrap().next().is_none() {
        println!("Downloading Tailwind CLI binaries...");
        
        // Run the download script
        let script_path = Path::new(&manifest_dir).join("scripts/download_tailwind_binaries.sh");
        let status = Command::new("bash")
            .arg(&script_path)
            .status()
            .expect("Failed to execute download_tailwind_binaries.sh");
            
        if !status.success() {
            panic!("Failed to download Tailwind CLI binaries");
        }
    }
}