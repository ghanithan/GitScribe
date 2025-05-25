# GitScribe

A desktop application for creating, managing, and publishing static websites directly from GitHub repositories.

## Features

- Dioxus-based desktop application
- Tailwind CSS styling
- Component architecture
- State management
- Theme support (light/dark mode)

## Project Structure

```
/src                   # Dioxus application source code
  /components          # Reusable UI components
  /pages               # Application pages/views
  /services            # GitHub API, file management services
  /utils               # Utility functions
/assets                # Static assets (icons, images)
/binaries              # Platform-specific Tailwind CLI binaries
/styles                # Tailwind CSS configuration and custom styles
/docs                  # Application documentation
```

## Development

### Prerequisites

- Rust (latest stable)

### Setup

1. Clone the repository
   ```
   git clone https://github.com/ghanithan/GitScribe.git
   cd GitScribe
   ```

2. Download Tailwind CLI binaries
   ```
   bash scripts/download_tailwind_binaries.sh
   ```

3. Build and run the application
   ```
   cargo run
   ```

   This will:
   - Build the Tailwind CSS styles using the Tailwind CLI binary for your platform
   - Launch the application

### Development Workflow

For active development with hot-reload:

```
cargo run
```

The application automatically builds Tailwind CSS on startup, so there's no need for separate Tailwind commands.

## Packaging

When packaging the application for distribution (as .dmg, .exe, .apk, .rpm, etc.), 
include only the Tailwind CLI binary that is relevant for the target platform in the `binaries` directory.

## License

MIT