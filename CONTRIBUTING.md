# Contributing to Conduit

Thank you for your interest in contributing to Conduit! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Issue Reporting](#issue-reporting)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone. Please be kind and constructive in your communications and contributions.

## Getting Started

1. **Fork the repository** on GitHub

2. **Clone your fork** to your local machine

   ```bash
   git clone https://github.com/YOUR-USERNAME/conduit.git
   cd conduit
   ```

3. **Set up the development environment**

   ```bash
   npm install
   ```

4. **Create a new branch** for your feature or bug fix

   ```bash
   git checkout -b feature/your-feature-name
   ```

## Project Structure

Conduit is organized as a Cargo workspace with multiple crates:

1. **Tauri Application** (`src-tauri/`): The main desktop application using Tauri, React, and TypeScript
2. **Backend Crate** (`conduit-backend/`): A reusable Rust crate that contains the core functionality

When contributing, be aware of which part of the project you're modifying:

- For frontend changes, work in the `src/` directory
- For Tauri-specific backend changes, work in the `src-tauri/src/` directory
- For core backend functionality, work in the `conduit-backend/src/` directory

## Development Workflow

1. **Make your changes** in your feature branch

2. **Test your changes** thoroughly

3. **Commit your changes** with clear, descriptive commit messages

   ```bash
   git commit -m "Add feature: description of the feature"
   ```

4. **Push your changes** to your fork

   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a pull request** from your fork to the main repository

## Pull Request Process

1. Ensure your code follows the project's coding standards
2. Update the README.md or documentation with details of changes if appropriate
3. Add relevant tests for your changes
4. Ensure all tests pass
5. The pull request will be reviewed by maintainers who may request changes
6. Once approved, your pull request will be merged

## Coding Standards

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code
- Run `clippy` to catch common mistakes and improve your code
- Write meaningful comments and documentation

### TypeScript/React

- Follow the [TypeScript coding guidelines](https://www.typescriptlang.org/docs/handbook/declaration-files/do-s-and-don-ts.html)
- Use functional components with hooks for React components
- Use ESLint and Prettier for code formatting

## Testing

- Write unit tests for new functionality
- Ensure existing tests pass before submitting a pull request
- For Rust code, use the built-in testing framework
- For TypeScript/React code, use Jest or the testing framework provided

## Documentation

- Update documentation for any changes to APIs or functionality
- Document new features thoroughly
- Use clear, concise language in documentation
- Include examples where appropriate

## Issue Reporting

When reporting issues, please include:

1. **Description** of the issue
2. **Steps to reproduce** the issue
3. **Expected behavior**
4. **Actual behavior**
5. **Environment details** (OS, browser, versions, etc.)
6. **Screenshots** or error logs if applicable

## Backend Crate

The `conduit-backend` crate contains the core functionality of Conduit, including:

- Memory management (create, retrieve, list, search, delete)
- API server implementation using Axum
- OpenAI-compatible endpoints

When contributing to the backend crate:

1. Ensure your changes maintain backward compatibility when possible
2. Add appropriate tests for new functionality
3. Update documentation comments for public APIs
4. Consider how your changes might affect other applications using this crate

## Memory Storage Location

As of recent changes, memory files are stored in the user's home directory under `~/.conduit/memories`. When working with memory-related features, be aware of this location to avoid issues with file access or permissions.

---

Thank you for contributing to Conduit! Your efforts help make this project better for everyone.
