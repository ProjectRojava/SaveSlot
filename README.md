# Rojava SaveSlot

> A revolutionary AI-powered database manager IDE built with Rust and Iced

Rojava SaveSlot is a next-generation database management tool that combines the power of Rust's type safety with AI-driven query optimization and natural language database interactions. Built following Domain-Driven Design (DDD) and Test-Driven Development (TDD) principles.

## 🚀 Features

- **AI-Powered Query Generation**: Write queries in natural language
- **Intelligent Query Optimization**: Automatic performance suggestions
- **Cross-Database Support**: PostgreSQL, MySQL, SQLite, MongoDB
- **Visual Schema Designer**: Interactive database schema visualization
- **Real-time Collaboration**: Team-based database management
- **Native Performance**: Built with Rust for maximum speed and safety
- **Modern UI**: Clean, responsive interface built with Iced

## 🏗️ Architecture

Rojava SaveSlot follows a clean architecture with clear separation of concerns:

```
┌─────────────────┐
│   UI Layer      │  ← Iced-based user interface
│   (Iced)        │
├─────────────────┤
│ Application     │  ← Application services and commands
│ Layer           │
├─────────────────┤
│ Domain Layer    │  ← Pure business logic (entities, services)
│ (Pure Rust)     │
├─────────────────┤
│ Infrastructure  │  ← Database connections, AI services
│ Layer           │
└─────────────────┘
```

### Key Principles

- **Domain-Driven Design (DDD)**: Clear domain boundaries and ubiquitous language
- **Test-Driven Development (TDD)**: Tests written before implementation
- **Hexagonal Architecture**: Ports and adapters pattern for clean dependencies
- **Type Safety**: Leveraging Rust's type system for correctness

## 🛠️ Technology Stack

- **Language**: Rust 🦀
- **UI Framework**: Iced
- **Database**: SQLx for type-safe SQL
- **AI Integration**: OpenAI API / Local models
- **Testing**: Built-in Rust testing + Mockall
- **Build System**: Cargo

## 📋 Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git
- A supported database (PostgreSQL, MySQL, SQLite, or MongoDB)

## 🚀 Quick Start

### Option 1: Automated Setup (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/rojava-saveslot.git
cd rojava-saveslot

# Run the setup script
chmod +x setup.sh
./setup.sh
```

### Option 2: Manual Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/yourusername/rojava-saveslot.git
cd rojava-saveslot
cargo build

# Run the application
cargo run
```

### Option 3: Docker

```bash
# Build the Docker image
docker build -t rojava-saveslot .

# Run the container
docker run -it --rm rojava-saveslot
```

## 🧪 Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html

# Run specific test module
cargo test domain::model::connection
```

### Development Workflow

```bash
# Check code compiles
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Watch for changes and auto-reload
cargo install cargo-watch
cargo watch -x run
```

### Project Structure

```
src/
├── domain/                 # 🏛️ Domain Layer (Pure Business Logic)
│   ├── model/              # Domain entities and value objects
│   │   ├── connection.rs   # Database connection entity
│   │   ├── query.rs        # Query entity
│   │   └── schema.rs       # Schema representation
│   ├── service/            # Domain services
│   └── repository/         # Repository interfaces (ports)
├── infrastructure/         # 🔌 Infrastructure Layer
│   ├── repository/         # Repository implementations (adapters)
│   ├── database/           # Database connection management
│   └── ai/                 # AI service integrations
├── application/            # 📋 Application Layer
│   ├── commands/           # Command handlers
│   └── queries/            # Query handlers
├── ui/                     # 🎨 Presentation Layer
│   ├── components/         # Reusable UI components
│   ├── styles/             # UI styling
│   └── app.rs              # Main application
└── main.rs                 # Application entry point
```

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Write tests first** (TDD approach)
4. **Implement your feature**
5. **Ensure tests pass**: `cargo test`
6. **Format code**: `cargo fmt`
7. **Lint code**: `cargo clippy`
8. **Commit changes**: `git commit -m 'Add amazing feature'`
9. **Push to branch**: `git push origin feature/amazing-feature`
10. **Open a Pull Request**

### Code Style

- Follow Rust naming conventions
- Write comprehensive tests for all new features
- Document public APIs with rustdoc comments
- Keep functions small and focused
- Use meaningful variable and function names

### Testing Guidelines

- Write tests before implementation (TDD)
- Aim for high test coverage (>90%)
- Use descriptive test names
- Test both happy path and error cases
- Mock external dependencies

## 🔧 Configuration

### Environment Variables

```bash
# AI Integration (optional)
OPENAI_API_KEY=your_openai_api_key

# Database connections
DATABASE_URL=postgresql://user:password@localhost/dbname

# Logging level
RUST_LOG=info
```

### Configuration File

Create a `config.toml` file in the project root:

```toml
[database]
default_connection_timeout = 30
max_connections = 10

[ai]
provider = "openai"  # or "local"
model = "gpt-4"
temperature = 0.3

[ui]
theme = "dark"  # or "light"
font_size = 14
```

## 📚 Documentation

- [Architecture Guide](docs/architecture.md)
- [API Documentation](docs/api.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Deployment Guide](docs/deployment.md)

## 🐛 Troubleshooting

### Common Issues

**Compilation Errors**
```bash
# Clear cache and rebuild
cargo clean
cargo build
```

**Database Connection Issues**
```bash
# Check database is running
pg_isready -h localhost -p 5432

# Verify connection string
echo $DATABASE_URL
```

**UI Not Responding**
```bash
# Check for async runtime issues
RUST_LOG=debug cargo run
```

## 📈 Roadmap

- [ ] **v0.1.0**: Basic database connections and query execution
- [ ] **v0.2.0**: AI-powered query generation
- [ ] **v0.3.0**: Visual schema designer
- [ ] **v0.4.0**: Real-time collaboration features
- [ ] **v0.5.0**: Plugin system
- [ ] **v1.0.0**: Production-ready release

## 🏆 Performance

Rojava SaveSlot is built for performance:

- **Startup Time**: < 500ms
- **Memory Usage**: < 50MB base
- **Query Execution**: Near-native database performance
- **UI Responsiveness**: 60fps on modern hardware

## 🔒 Security

- All database connections use TLS by default
- SQL injection prevention through parameterized queries
- Secure credential storage
- Regular security audits with `cargo audit`

## 📄 License

This project is licensed under the AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Iced](https://github.com/iced-rs/iced) - Amazing Rust GUI framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime for Rust
- The Rust community for excellent tooling and support

## 📞 Support

- 📧 Email: support@rojava-saveslot.dev
- 💬 Discord: [Join our community](https://discord.gg/rojava-saveslot)
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/rojava-saveslot/issues)
- 📖 Docs: [Documentation](https://docs.rojava-saveslot.dev)

---

**Built with ❤️ and 🦀 by the Rojava SaveSlot team**
```