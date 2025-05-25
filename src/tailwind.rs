use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

// Include the Tailwind CLI binaries at compile time
// The actual binaries will be embedded during build using build.rs
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const TAILWIND_BINARY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tailwindcss-binaries/tailwindcss-linux-x64"));

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const TAILWIND_BINARY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tailwindcss-binaries/tailwindcss-linux-arm64"));

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const TAILWIND_BINARY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tailwindcss-binaries/tailwindcss-macos-x64"));

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const TAILWIND_BINARY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tailwindcss-binaries/tailwindcss-macos-arm64"));

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const TAILWIND_BINARY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tailwindcss-binaries/tailwindcss-windows-x64.exe"));

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
const TAILWIND_BINARY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tailwindcss-binaries/tailwindcss-windows-arm64.exe"));

pub struct TailwindCli {
    binary_path: PathBuf,
}

impl TailwindCli {
    pub fn new() -> Self {
        let binary_path = extract_tailwind_binary();
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

    pub fn watch(&self, input_path: &Path, output_path: &Path, config_path: &Path) -> Result<(), String> {
        self.run_command(&[
            "-i", input_path.to_str().unwrap(),
            "-o", output_path.to_str().unwrap(),
            "--watch",
            "-c", config_path.to_str().unwrap(),
        ])
    }

    fn run_command(&self, args: &[&str]) -> Result<(), String> {
        let mut command = if cfg!(target_os = "windows") {
            Command::new(&self.binary_path)
        } else {
            let mut cmd = Command::new(&self.binary_path);
            cmd.arg("-c");
            cmd
        };

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

fn extract_tailwind_binary() -> PathBuf {
    // Create a temporary directory for the binary
    let temp_dir = env::temp_dir().join("gitscribe-tailwind");
    fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");

    // Create the binary file path
    let binary_name = if cfg!(target_os = "windows") {
        "tailwindcss.exe"
    } else {
        "tailwindcss"
    };
    let binary_path = temp_dir.join(binary_name);

    // Write the binary to the temporary file
    let mut file = fs::File::create(&binary_path).expect("Failed to create temporary binary file");
    file.write_all(TAILWIND_BINARY).expect("Failed to write binary data");

    // Make the binary executable on Unix-like systems
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary_path).expect("Failed to get file metadata").permissions();
        perms.set_mode(0o755); // rwx for owner, rx for group and others
        fs::set_permissions(&binary_path, perms).expect("Failed to set file permissions");
    }

    binary_path
}

// Function to build Tailwind CSS at application startup
pub fn build_tailwind_css() -> Result<(), String> {
    let tailwind = TailwindCli::new();
    
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input_path = project_dir.join("styles/input.css");
    let output_path = project_dir.join("styles/output.css");
    let config_path = project_dir.join("tailwind.config.json");
    
    tailwind.build(&input_path, &output_path, &config_path)
}