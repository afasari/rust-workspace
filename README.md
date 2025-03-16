# 🦀 Rust Workspace Projects

A collection of Rust projects showcasing various implementations and learning experiences.

## 📂 Projects

### Todo API

A RESTful API built with Axum and Diesel for managing todos.

- **Tech Stack**: Axum, Diesel, PostgreSQL
- **Features**: CRUD operations, database migrations, integration tests
- [View Project](./todo)

## 🛠️ Setup

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- Diesel CLI

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

## 🧪 Testing

Each project contains its own test suite including:

- Unit tests
- Integration tests
- E2E tests

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📬 Contact

- GitHub: @afasari
⭐ Don't forget to star this repository if you find it helpful!

## 🗺️ Roadmap

### Workspace Level

- [ ] Add more Rust projects
  - [ ] CLI tools
  - [ ] Web servers
  - [ ] System utilities
- [ ] Implement shared libraries
- [ ] Add benchmarking suite
- [ ] Add workspace-wide documentation
- [ ] Set up cross-platform CI/CD
- [ ] Add performance monitoring
- [ ] Create development guidelines

### Current Projects

#### Todo API Roadmap

- [ ] Add user authentication
- [ ] Implement rate limiting
- [ ] Add task categories
- [ ] Implement task priorities
- [ ] Add due dates
- [ ] Create API documentation
- [ ] Add metrics monitoring
- [ ] Implement caching
- [ ] Add WebSocket support
- [ ] Create frontend application
  - [ ] React.js implementation
  - [ ] Mobile responsive design
  - [ ] Real-time updates
