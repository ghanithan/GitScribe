use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

// Tailwind CLI structure to manage execution
pub struct TailwindCli {
    binary_path: PathBuf,
}

impl TailwindCli {
    pub fn new() -> Self {
        let binary_path = get_tailwind_binary_path();
        Self { binary_path }
    }

    pub fn build(&self, input_path: &Path, output_path: &Path, config_path: &Path) -> Result<(), String> {
        self.run_command(&[
            "-i", input_path.to_str().unwrap(),
            "-o", output_path.to_str().unwrap(),
            "--minify",
            "-c", config_path.to_str().unwrap(),
        ])
    }

    pub fn watch(&self, input_path: &Path, output_path: &Path, config_path: &Path) -> Result<thread::JoinHandle<Result<(), String>>, String> {
        let input_path = input_path.to_path_buf();
        let output_path = output_path.to_path_buf();
        let config_path = config_path.to_path_buf();
        let binary_path = self.binary_path.clone();
        
        // Run Tailwind watch in a separate thread
        let handle = thread::spawn(move || {
            let args = vec![
                "-i", input_path.to_str().unwrap(),
                "-o", output_path.to_str().unwrap(),
                "--watch",
                "-c", config_path.to_str().unwrap(),
            ];
            
            let mut command = Command::new(&binary_path);
            let status = command
                .args(&args)
                .status()
                .map_err(|e| format!("Failed to execute Tailwind CLI: {}", e))?;
                
            if status.success() {
                Ok(())
            } else {
                Err(format!("Tailwind CLI exited with status: {}", status))
            }
        });
        
        Ok(handle)
    }

    fn run_command(&self, args: &[&str]) -> Result<(), String> {
        let mut command = Command::new(&self.binary_path);
        
        let status = command
            .args(args)
            .status()
            .map_err(|e| format!("Failed to execute Tailwind CLI: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Tailwind CLI exited with status: {}", status))
        }
    }
}

fn get_tailwind_binary_path() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR");
    let binaries_dir = Path::new(&manifest_dir).join("binaries");
    
    // Determine which binary to use based on current platform
    let binary_name = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "tailwindcss-linux-x64"
    } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
        "tailwindcss-linux-arm64"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "tailwindcss-macos-x64"
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "tailwindcss-macos-arm64"
    } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "tailwindcss-windows-x64.exe"
    } else if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
        "tailwindcss-windows-arm64.exe"
    } else {
        panic!("Unsupported platform for Tailwind CLI");
    };
    
    binaries_dir.join(binary_name)
}

// Function to build Tailwind CSS at application startup
pub fn build_tailwind_css() -> Result<(), String> {
    let tailwind = TailwindCli::new();
    
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let input_path = project_dir.join("styles/input.css");
    let output_path = project_dir.join("styles/output.css");
    let config_path = project_dir.join("tailwind.config.json");
    
    tailwind.build(&input_path, &output_path, &config_path)
}