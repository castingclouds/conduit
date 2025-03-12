# Conduit

Conduit is a modern desktop application built with Tauri, React, and TypeScript. It provides an OpenAI-compatible API server with memory management capabilities.

## Features

- **OpenAI-Compatible API**: Implements standard OpenAI API endpoints including `/v1/models` and `/v1/chat/completions`
- **Memory Management**: Create, retrieve, list, search, and delete memories
- **User-Specific Storage**: Memories are stored in the user's home directory (`~/.conduit/memories`)
- **Modern UI**: Built with React and TypeScript for a responsive user experience
- **Reusable Backend**: The Rust backend is packaged as a separate crate that can be used in other applications

## Architecture

- **Frontend**: React with TypeScript, running in a Tauri webview
- **Backend**: Rust-based API server using Axum framework, packaged as a reusable crate
- **Storage**: File-based memory storage in the user's home directory
- **Workspace Structure**: Cargo workspace with separate crates for the Tauri application and backend

## Recent Changes

### Backend Crate Packaging

The Rust backend has been extracted into a separate crate called `conduit-backend`. This allows the backend to be used in other applications, not just the Tauri frontend.

Benefits of this change:

- **Reusability**: The backend can now be used in any Rust application
- **Maintainability**: Better code organization with clear separation of concerns
- **Testability**: The backend can be tested independently of the Tauri application
- **Flexibility**: Other applications can use just the parts they need

### Memory Storage Location

The memory storage location has been moved from the source code directory to the user's home directory under `~/.conduit/memories`. This change helps prevent the Tauri application from restarting due to file changes in the source code directory when new memory files are created.

Benefits of this change:

- Prevents unintended application restarts during development
- Follows best practices for application data storage
- Separates application data from source code
- Ensures persistence of memories across application updates

## Installation

### Development Build

```bash
# Clone the repository
git clone <repository-url>
cd conduit

# Install dependencies
npm install

# Start the development server
npm run tauri dev
```

### Production Build

```bash
# Build for production
npm run tauri build
```

The built application will be available in the `src-tauri/target/release` directory.

## Usage

### API Testing

The application includes an API testing interface that allows you to:

1. Test the `/v1/models` endpoint to view available models
2. Test the `/v1/chat/completions` endpoint to interact with AI models
3. Create, list, and delete memories through the `/v1/memories` endpoints

To access the API testing interface, click the "Show API Test" button on the main screen.

### Memory Management

Memories are stored as markdown files in the `~/.conduit/memories` directory. Each memory includes:

- A unique ID
- A title
- Content
- Tags for categorization
- Creation and update timestamps

## Contributing

Contributions to Conduit are welcome! Here are some ways you can contribute:

- Report bugs and request features by creating issues
- Submit pull requests for bug fixes or new features
- Improve documentation
- Share your ideas for improving the application

Please see our [CONTRIBUTING.md](CONTRIBUTING.md) file for detailed guidelines on how to contribute to this project, including:

- Development workflow
- Pull request process
- Coding standards
- Testing requirements
- Documentation guidelines

## License

Conduit is licensed under the [MIT License](LICENSE.md). See the [LICENSE.md](LICENSE.md) file for details.

## Using the Backend in Other Applications

The `conduit-backend` crate can be used in any Rust application. Here's a simple example:

```rust
use conduit_backend::ConduitBackend;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Initialize the backend (uses ~/.conduit/memories by default)
    let backend = ConduitBackend::new(None)?;
    
    // Create a memory
    let memory_id = backend.create_memory(
        "Test Memory".to_string(),
        "This is a test memory.".to_string(),
        vec!["test".to_string(), "example".to_string()]
    )?;
    
    println!("Created memory with ID: {}", memory_id);
    
    // Start the API server if needed
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    backend.start_server(addr).await?;
    
    Ok(())
}
```

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) (v14 or later)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) with the following extensions:
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [TypeScript and JavaScript Language Features](https://marketplace.visualstudio.com/items?itemName=vscode.typescript-language-features)
