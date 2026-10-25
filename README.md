# 🚀 Ralert - Lightweight Network Operations Center

![Project Status](https://img.shields.io/badge/status-in_development-yellow?style=for-the-badge)
![Built With](https://img.shields.io/badge/built_with-Rust-orange?style=for-the-badge&logo=rust)
![Architecture](https://img.shields.io/badge/architecture-clean-blue?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

The foundation of a **lightweight Network Operations Center (NOC)**, built with Rust following Clean Architecture principles. The goal is monitoring services with minimal resource consumption.

> **Current status:** the API foundation is implemented: Clean Architecture layout, JWT sign-in, role-based access, a health check and automatic SQLite migrations. Service monitoring, alerts and the dashboard are in the Roadmap section below and are not implemented yet.

---

## 🎯 What is Ralert?

**Ralert** is a micro-NOC designed to monitor services, APIs, and infrastructure with minimal CPU and memory footprint. Ideal for running on resource-constrained environments like Raspberry Pi, small VPS instances, or alongside other applications.

### ✨ Key Features

- 🔐 **JWT Authentication** - Secure user authentication with role-based access control
- 🏗️ **Clean Architecture** - Maintainable, testable, and scalable codebase
- 💾 **SQLite Database** - Lightweight persistence with automatic migrations
- 🚀 **High Performance** - Built with Rust and Tokio for async operations
- 🔒 **Security First** - Bcrypt password hashing, JWT tokens, protected routes
- 📊 **RESTful API** - Well-designed HTTP endpoints following best practices

---

## 🏗️ Architecture

Ralert follows **Clean Architecture** principles with strict separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│ PRESENTATION LAYER (HTTP Handlers, Extractors)         │
│ - Axum web framework integration                       │
│ - Request/Response handling                            │
└────────────────┬────────────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────────────┐
│ APPLICATION LAYER (Use Cases, Mappers)                 │
│ - Business logic implementation                        │
│ - Data transformation                                  │
└────────────────┬────────────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────────────┐
│ DOMAIN LAYER (Entities, Traits, DTOs)                  │
│ - Core business rules                                  │
│ - Framework-agnostic                                   │
└─────────────────────────────────────────────────────────┘
                 ▲
┌────────────────┴────────────────────────────────────────┐
│ INFRASTRUCTURE LAYER (Database, External Services)     │
│ - SQLite with SQLx                                     │
│ - JWT service                                          │
│ - Middleware                                           │
└─────────────────────────────────────────────────────────┘
```

### 📁 Project Structure

```
src/
├── domain/              # Business logic core
│   ├── entities/        # Domain entities (UserEntity, etc.)
│   ├── dtos/           # Data Transfer Objects
│   ├── repositories/   # Repository traits
│   ├── datasources/    # Datasource traits
│   └── use_cases/      # Use case traits
├── application/         # Use case implementations
│   ├── use_cases/      # Business logic implementations
│   ├── mappers/        # DTO ↔ Entity transformations
│   └── dtos/           # Row DTOs for database
├── infrastructure/      # External concerns
│   ├── database/       # Database connection & migrations
│   ├── datasources/    # Database implementations
│   ├── repositories/   # Repository implementations
│   ├── services/       # External services (JWT, etc.)
│   ├── middleware/     # Authentication middleware
│   └── di/             # Dependency injection container
└── presentation/        # HTTP layer
    ├── web/            # HTTP handlers
    └── extractors/     # Custom request extractors
```

---

## 🚀 Quick Start

### Prerequisites

- **Rust** (edition 2024 or later)
- **SQLite3**
- **Cargo**

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/pxnditxyr/ralert.git
   cd ralert
   ```

2. **Set up environment variables:**
   ```bash
   cp .example.dev.env .env
   ```

3. **Configure your `.env` file:**
   ```env
   SERVER_PORT=8003
   DATABASE_URL=sqlite:./ralert.db
   JWT_SECRET=your-super-secret-jwt-key-change-in-production
   LOG_LEVEL=info
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```

   The server will start at `http://localhost:8003`

### 🎬 First Steps

**1. Sign in to get a JWT token:**
```bash
curl -X POST http://localhost:8003/api/v1/auth/sign-in \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@ralert.com",
    "password": "Admin123!"
  }'
```

**2. Use the token to access protected endpoints:**
```bash
TOKEN="your-jwt-token-here"

curl http://localhost:8003/api/v1/profile \
  -H "Authorization: Bearer $TOKEN"
```

---

## 📡 API Endpoints

### Public Endpoints (No Authentication)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check endpoint |
| `POST` | `/api/v1/auth/sign-in` | User authentication |

### Protected Endpoints (Requires JWT)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/profile` | Get authenticated user profile |

### Admin Only Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/admin/test` | Admin test endpoint |

---

## 🔐 Default Credentials

⚠️ **For development only - Change these in production!**

- **Admin User:**
  - Email: `admin@ralert.com`
  - Password: `Admin123!`

- **Regular User:**
  - Email: `user@ralert.com`
  - Password: `Admin123!`

---

## 🛠️ Tech Stack

### Core Technologies
- **Language:** Rust
- **Async Runtime:** Tokio
- **Web Framework:** Axum
- **Database:** SQLite with SQLx
- **Authentication:** JWT with bcrypt

### Key Dependencies
- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database toolkit
- `jsonwebtoken` - JWT implementation
- `bcrypt` - Password hashing
- `serde` - Serialization/deserialization
- `uuid` - Unique identifiers

---

## 🧪 Development

### Run in development mode
```bash
cargo run
```

### Build for production
```bash
cargo build --release
```

### Run tests
```bash
cargo test
```

### Check code quality
```bash
cargo clippy --all-targets
```

### Format code
```bash
cargo fmt
```

### Watch mode (auto-reload)
```bash
cargo install cargo-watch
cargo watch -x run
```

---

## �� Database Migrations

Migrations are automatically applied on startup. They are located in the `migrations/` directory.

### Create a new migration
```bash
# Manual approach
touch migrations/$(date +%Y%m%d%H%M%S)_description.sql
```

### Migration naming convention
```
YYYYMMDDHHMMSS_description.sql
```

Example:
```
20241104120000_create_users_table.sql
```

---

## 🐳 Docker Support

### Build Docker image
```bash
docker build -t ralert:latest .
```

---

## 🗺️ Roadmap

### ✅ Completed
- [x] Clean Architecture implementation
- [x] JWT authentication system
- [x] User management with roles
- [x] Protected routes middleware
- [x] SQLite database integration
- [x] Password hashing with bcrypt

### 🚧 In Progress
- [ ] Service monitoring (HTTP, TCP, Ping)
- [ ] Real-time updates with WebSockets
- [ ] Alert management system

### 📋 Planned Features
- [ ] React dashboard
- [ ] Service uptime tracking
- [ ] Metrics collection and visualization
- [ ] Background health check jobs
- [ ] Notification integrations (Slack, Discord, Email)
- [ ] Multi-user support with permissions
- [ ] API key authentication
- [ ] Prometheus metrics export
- [ ] Rate limiting
- [ ] Comprehensive logging with tracing

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines
1. Follow Clean Architecture principles
2. Maintain separation of concerns
3. Write tests for new features
4. Keep dependencies to a minimum
5. Use conventional commits

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Author

**Ralert** was created by [**Pxndxs**](https://github.com/pxnditxyr) 🐼

- Website: [pxndxs.com](https://pxndxs.com)
- GitHub: [@pxnditxyr](https://github.com/pxnditxyr)

---

## 🙏 Acknowledgments

- The Rust community for excellent tools and libraries
- Clean Architecture by Robert C. Martin
- Hexagonal Architecture principles

---

## 📸 Screenshots

*Coming soon! Dashboard and monitoring views will be added once the frontend is implemented.*

---

## ⚡ Performance goals

Ralert is designed for minimal resource consumption. These are design targets and have **not been benchmarked yet**:

- **Memory footprint:** ~10-20MB idle
- **CPU usage:** <1% idle
- **Startup time:** <100ms
- **Response time:** <5ms for most endpoints

---

## 🔒 Security

- ✅ JWT-based authentication
- ✅ Bcrypt password hashing
- ✅ Role-based access control (RBAC)
- ✅ SQL injection protection (SQLx prepared statements)
- ⚠️ Rate limiting (planned)
- ⚠️ Request size limits (planned)

---

## 📖 Documentation

### HTTP Examples

See the `examples/http/` directory for request examples:
- `examples/http/auth.http` - Authentication examples
- `examples/http/protected.http` - Protected endpoint examples
- `examples/http/admin_only.http` - Admin endpoint examples

### Architecture Decisions

The project follows Clean Architecture with these key decisions:
1. **Domain Layer** is framework-agnostic (uses minimal external dependencies)
2. **DTOs** are in the Domain layer (they define use case contracts)
3. **Serde** is used in Domain (acceptable in Rust ecosystem)
4. **Repositories** work with Entities, not DTOs
5. **Datasources** handle database-specific operations

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

**⭐ Star this repo if you find it useful!**

</div>
