# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a blog backend API built with Rust and the Axum web framework. It provides REST endpoints for authentication, comments, and content management with MongoDB as the database.

## Key Technologies

- **Framework**: Axum 0.7.9 (async web framework)
- **Database**: MongoDB 3.2.3 with native Rust driver
- **Authentication**: JWT with bcrypt password hashing
- **Runtime**: Tokio async runtime
- **Containerization**: Docker with multi-stage builds

## Common Development Commands

### Building and Running
```bash
# Local development
cargo run

# Production build
cargo build --release

# Run with Docker Compose (includes MongoDB)
docker-compose up

# Run tests
cargo test

# Run a specific test
cargo test test_name
```

### Code Quality
```bash
# Format code
cargo fmt

# Check linting
cargo clippy

# Check types
cargo check
```

## Architecture

### Project Structure
- `src/controllers/` - HTTP handlers and route definitions
- `src/models/` - Data models (User, Comment) with MongoDB integration
- `src/auth/` - JWT authentication middleware and extractors
- `src/utils/` - Utilities for encryption, logging, Discord webhooks
- `src/env/` - Environment configuration and AppState management

### Key Architectural Patterns

1. **State Management**: The `AppState` struct (src/env/state.rs) manages shared resources like database connections and JWT secrets, injected through Axum's state system.

2. **Authentication Flow**: 
   - JWT tokens stored in secure cookies
   - Custom extractors (`AuthUser`, `RootUser`) for role-based access control
   - Authentication middleware validates tokens and extracts user context

3. **Database Access**: Direct MongoDB driver usage with BSON serialization. No ORM layer - queries are built manually for performance.

4. **Error Handling**: Result-based error propagation with custom error responses mapped to appropriate HTTP status codes.

5. **Async Architecture**: Fully async/await with Tokio runtime, all I/O operations are non-blocking.

## API Endpoints

### Authentication
- `POST /auth/signIn` - User login
- `POST /auth/signUp` - User registration
- `POST /auth/signOut` - User logout
- `GET /auth/me` - Get current user

### Comments
- `GET /comment/list` - List comments for a post
- `POST /comment/create` - Create comment (authenticated users get Root badge)
- `DELETE /comment/:id` - Delete comment (root only)

### Recent Content
- `GET /recent` - Get recent blog posts metadata

## Environment Configuration

Required environment variables (see `.env.example`):
- `MONGODB_URI` - MongoDB connection string
- `JWT_SECRET` - Secret for JWT signing
- `JWT_MAXAGE` - Token expiration time
- `DISCORD_WEBHOOK_*` - Discord integration webhooks
- `FRONTEND_ORIGIN` - CORS allowed origin

## Security Considerations

- Passwords hashed with bcrypt (cost factor 12)
- JWT tokens with HMAC signing
- Secure cookie flags (HttpOnly, Secure, SameSite)
- CORS configured for specific origins
- Role-based access control (User vs Root roles)

## Testing

Tests are located in `src/controllers/__tests__/`. Run specific test modules:
```bash
cargo test auth_tests
cargo test comments_tests
```

## Docker Deployment

The project uses multi-stage Docker builds for optimization:
```bash
# Build and run with Docker Compose
docker-compose up --build

# Access API at http://localhost:8081
# MongoDB runs on internal network
```

## Common Development Tasks

When implementing new endpoints:
1. Define the route handler in appropriate controller module
2. Add data models to `src/models/` if needed
3. Register the route in `src/main.rs` router configuration
4. Add authentication extractors if endpoint requires auth
5. Write integration tests in `controllers/__tests__/`

When modifying authentication:
- JWT logic is in `src/auth/jwt.rs`
- Custom extractors in `src/auth/extractors.rs`
- Cookie handling in auth controllers

When working with MongoDB:
- Database operations use the async MongoDB driver
- BSON serialization happens automatically via Serde
- Collection names and indexes defined in model modules