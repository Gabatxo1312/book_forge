# 📚 BookForge

**BookForge** is an open source self-hosted shared library web application built with Rust. Perfect for groups of friends who want to share a common book library, it allows users to manage and share their book collections within a community.

## ✨ Features

- 📖 **Collection Management**: Add and organize your personal books
- 🔄 **Lending System**: Lend and borrow books between users
- 🔍 **Advanced Search**: Search by title, owner, or current holder
- 🌐 **OpenLibrary Integration**: Automatic book metadata retrieval
- 👥 **Multi-user Support**: Multiple users with owner and holder management
- 🎨 **Modern Interface**: Responsive web interface with Tailwind CSS and Alpine.js

## 🛠️ Tech Stack

- **Backend**: Rust with Axum (async web framework)
- **Database**: SQLite with SeaORM (ORM)
- **Frontend**: Askama templates with Tailwind CSS and Alpine.js
- **External API**: OpenLibrary integration for book metadata (WIP)
- **Migrations**: SeaORM Migration for database schema management

## 🚀 Installation & Setup

### Prerequisites

- Rust (version 1.70+)
- SQLite3

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Gabatxo1312/book_forge.git
   cd book_forge
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Configure the database**

   The `BookForge.toml` file contains the database configuration:
   ```toml
   database_url="sqlite://db.sqlite?mode=rwc"
   ```

4. **Start the application**
   ```bash
   cargo run
   ```

The application will be available at `http://localhost:3001`

## 🏗️ Project Architecture

```
book_forge/
├── src/                   # Main source code
│   ├── config/            # Application configuration
│   ├── handlers/          # HTTP route handlers
│   ├── helpers/           # Utilities and filters
│   ├── services/          # Services (OpenLibrary API)
│   └── main.rs            # Application entry point
├── entity/                # SeaORM entities
├── migration/             # Database migrations
├── templates/             # HTML templates (Askama)
```

## 🔧 Development

### Adding a migration

```bash
cd migration
cargo run -- generate MIGRATION_NAME
```

Note: Migrations run on database initialization (when run `cargo run`)

### Logging and debugging

The application uses `tracing` for logging. You can adjust the log level with environment variables:

```bash
RUST_LOG=debug cargo run
```

## 🤝 Contributing

Contributions are welcome! To contribute:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add: amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the [AGPL-3.0](LICENSE) License. You are free to use, modify, and distribute it under the terms of the GNU Affero General Public License v3.

## 🙏 Acknowledgments

- [OpenLibrary](https://openlibrary.org/) for the book metadata API
- The Rust community for excellent tools and frameworks
- [SeaORM](https://www.sea-ql.org/SeaORM/) for the ORM
- [Axum](https://github.com/tokio-rs/axum) for the web framework

## 📞 Support

If you encounter any issues or have questions, feel free to:
- Open an [issue](https://github.com/Gabatxo1312/book_forge/issues)
- Check the documentation of dependencies
- Join project discussions

---

**BookForge** - Share your books, enrich your community ! 📚✨
