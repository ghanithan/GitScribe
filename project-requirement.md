# GitScribe Project Requirements

## Overview
GitScribe is an open-source desktop (and future mobile) application that enables users to create, manage, and publish static websites (blogs, portfolios, etc.) directly from their GitHub repositories. The app integrates with GitHub using a GitHub App for authentication and repository management.

## Core Features
- **GitHub Integration:**
  - Primary login via GitHub (OAuth/GitHub App).
  - Manages a repository for each user's site.
- **Content Creation:**
  - Users can create articles/posts as Markdown (`.md`) files.
  - Articles are saved directly to the managed GitHub repo.
  - Side-by-side Markdown editor and live preview.
- **Site Structure:**
  - Supports articles, portfolio pages, and custom pages.
  - Option to write code or add extensions (for JAMstack customization).
- **Themes & Builders:**
  - Users can choose from a set of themes.
  - Support for multiple JAMstack static site generators (e.g., Eleventy, Next.js, Astro).
  - Extensible to add more builders.
- **Deployment Options:**
  - Choose deployment target: GitHub Pages, Cloudflare Pages, Vercel, or Blob bucket.
  - Extensible to support more deployment providers.
  - Automated setup of GitHub Actions for CI/CD and deployment.
- **Privacy & Data:**
  - No user data is stored by the app except for crash reports.

## Future Extensions
- Mobile application with similar features.
- Marketplace for themes and JAMstack extensions.
- Advanced portfolio and code snippet management.

## Technology Stack
- **Frontend Framework:** Dioxus Lab (Rust-based cross-platform UI framework)
- **Styling:** Tailwind CSS (utility-first CSS framework)
- **Language:** Rust
- **Target Platforms:** Desktop (Windows, macOS, Linux) with future mobile support

## Technical Implementation Details

### Core Pipeline
| Stage | Technology | Description |
|-------|------------|-------------|
| File Management | `ignore` | Recursively discover and manage Markdown files and assets |
| Markdown Processing | `pulldown-cmark` | Fast, CommonMark-compatible Markdown parsing |
| Template Engine | `handlebars-rust` | Front-matter (YAML/TOML) + global site data merging |
| CSS Processing | Tailwind CLI | Standalone CLI for CSS generation with minification |
| HTML Processing | DOM manipulation | Plugin injection, canonical URLs, and metadata |
| File System | `std::fs` | Deterministic file handling with cache-busting |
| Preview Server | `axum` | Local development server with live-reload via SSE |

### Plugin System
- JavaScript-based plugin architecture
- TOML manifest for plugin configuration
- Support for head/tail script injection
- Asset management and copying
- Order-controlled plugin execution

### Performance Targets
- Binary size: < 15 MB (stripped)
- Memory usage: 30-60 MB during development
- Cross-platform support (x64, ARM)
- Offline-first operation

## Project Structure

### GitScribe Application Structure
```
/src                   # Dioxus application source code
  /components          # Reusable UI components
  /pages              # Application pages/views
  /services           # GitHub API, file management services
  /utils              # Utility functions
/assets               # Static assets (icons, images)
/styles               # Tailwind CSS configuration and custom styles
/docs                 # Application documentation
Cargo.toml            # Rust dependencies and project configuration
Dioxus.toml           # Dioxus configuration
tailwind.config.js    # Tailwind CSS configuration
```

### User's Generated Site Structure (Created by GitScribe)
This is the structure that GitScribe will create in the user's GitHub repository:
```
/.gitscribe.yml         # site config (build + deploy settings)
/src                   # markdown/blog posts & asset folder
/docs                  # portfolio pages, components
/.github/workflows     # CI: build on push, deploy to GitHub Pages/Cloudflare/Vercel
```

## Implementation Phases

### Phase 1: Core Application Setup
1. Set up Dioxus project with Tailwind CSS
2. Implement GitHub OAuth authentication
3. Create basic UI components and layout

### Phase 2: Content Management
1. Implement Markdown editor with live preview
2. Add file management for GitHub repository
3. Create site structure management

### Phase 3: Theme & Builder Integration
1. Implement theme selection and customization
2. Add support for multiple static site generators
3. Create builder configuration UI

### Phase 4: Deployment
1. Implement deployment target configuration
2. Add GitHub Actions workflow generation
3. Create deployment status monitoring

## Security & Offline Features
- No Node.js dependency, eliminating supply-chain risks
- User permission-based execution
- Optional sandbox for external processes
- Offline-first operation with sync capabilities
- Secure plugin system with controlled execution

## Known Limitations
- Static-only rendering (no hybrid/ISR)
- No built-in CMS (can be added via plugins)
- Interactive features require manual JS plugin implementation
- Initial focus on desktop platforms 