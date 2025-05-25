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

### Setup

1. Clone the repository
   ```
   git clone https://github.com/ghanithan/GitScribe.git
   cd GitScribe
   ```

2. Build and run the application
   ```
   cargo run
   ```

   This will:
   - Download the appropriate Tailwind CLI binaries during the build process (first run only)
   - Build the Tailwind CSS styles
   - Launch the application

### Development Workflow

For active development with hot-reload:

```
cargo run
```

The application automatically builds Tailwind CSS on startup, so there's no need for separate Tailwind commands.

## License

MIT