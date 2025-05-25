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
/styles                # Tailwind CSS configuration and custom styles
/docs                  # Application documentation
```

## Development

### Prerequisites

- Rust (latest stable)
- curl (for downloading Tailwind CSS CLI)

### Setup

1. Clone the repository
   ```
   git clone https://github.com/ghanithan/GitScribe.git
   cd GitScribe
   ```

2. Install dependencies
   ```
   cargo build
   ```

3. Build Tailwind CSS
   ```
   ./scripts/setup_tailwind.sh build
   ```

4. Run the application
   ```
   cargo run
   ```

### Development Workflow

For active development with hot-reload:

1. Start Tailwind CSS watcher
   ```
   ./scripts/setup_tailwind.sh watch
   ```

2. Run with hot-reload
   ```
   cargo run
   ```

## License

MIT