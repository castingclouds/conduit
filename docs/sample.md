---
id: 550e8400-e29b-41d4-a716-446655440000
title: Getting Started with Conduit
tags: [guide, introduction, openai]
created_at: 2025-03-11T19:45:00Z
updated_at: 2025-03-11T19:45:00Z
---

# Getting Started with Conduit

Conduit is an OpenAI API-compatible backend built with Rust and Tauri, designed to provide local AI capabilities with a familiar API interface.

## Key Features

1. **OpenAI-compatible API**: Use the same client libraries and code you would use with OpenAI.
2. **Local Memory Storage**: All your data stays on your machine, stored as Markdown files.
3. **Tauri Integration**: Seamlessly integrates with your Tauri application.

## API Endpoints

The API mimics OpenAI's endpoints:

- `/v1/chat/completions` - For chat-based interactions
- `/v1/embeddings` - For generating vector embeddings
- `/v1/models` - To list available models

## Memory System

Conduit stores all memories as Markdown files with YAML frontmatter, making them human-readable and easy to manage manually if needed.

Each memory file contains:
- Unique ID
- Title
- Content (in Markdown)
- Tags for organization
- Creation and update timestamps

## Getting Started

To use Conduit in your application, simply point your OpenAI client to the local API endpoint and start making requests as you normally would with the OpenAI API.
