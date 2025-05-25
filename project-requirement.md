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

## Project Structure (Proposed)
```
/.gitscribe.yml         # site config (build + deploy settings)
/src                   # markdown/blog posts & asset folder
/docs                  # portfolio pages, components
/.github/workflows     # CI: build on push, deploy to GitHub Pages/Cloudflare/Vercel
``` 