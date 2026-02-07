# CLAUDE.md - Wedding Planner SaaS Project

## 🎯 Project Context

This is a **learning + product project** to:
1. Learn **Rust** programming (first time, coming from Go/PHP background)
2. Learn **DevOps/CI-CD** practices with multiple servers
3. Learn **gRPC / RPC** patterns and when to use them vs REST
4. Build a **high-concurrency** wedding planner platform that can be monetized as a SaaS product

## 👤 About the Developer

- **Experience:** Go, PHP (legacy framework), wants to learn modern tech
- **Learning Style:** Challenge mode - learn by doing real projects
- **IDE:** RustRover
- **Subscription:** Claude Pro

## 🏗️ Tech Stack

| Layer | Technology | Status |
|-------|------------|--------|
| Backend | **Rust + Actix-web** | Learning |
| Frontend | **Vue 3 + Vite** (SPA) | Familiar territory |
| Database | **PostgreSQL** | Experienced |
| Cache / NoSQL | **Redis** (caching, sessions, rate limiting) | Familiar |
| NoSQL (optional) | **MongoDB** (flexible vendor data, event logs) | Learning |
| API | **REST** (public API) + **gRPC** (internal service-to-service) | Learning |
| Hosting | Multiple servers (free tier) | Learning |
| CI/CD | GitHub Actions | Learning |

## 🔀 API Strategy: REST vs gRPC

### REST API (Primary — client-facing)
- All frontend ↔ backend communication
- Public API for third-party integrations
- Standard JSON over HTTP

### gRPC (Learning goal — internal services)
- Service-to-service communication (e.g., API ↔ Crawler, API ↔ Notification service)
- High-throughput scenarios (bulk guest imports, real-time RSVP updates)
- Strongly typed contracts via Protocol Buffers

### When to use which:
| Scenario | Use | Why |
|----------|-----|-----|
| Frontend calls backend | REST | Browser-friendly, JSON, simple |
| Crawler sends data to API | gRPC | Fast, typed, internal only |
| Notification service | gRPC | Low-latency, streaming support |
| Webhook / third-party | REST | Universal compatibility |
| Real-time RSVP dashboard | gRPC streaming or WebSocket | Live updates |

## ⚡ High Concurrency Goals

This project should be designed for scale from the start:

1. **Connection pooling** — Use `deadpool` or `bb8` for PostgreSQL + Redis connections
2. **Async everywhere** — Tokio runtime with Actix-web, non-blocking I/O
3. **Caching strategy** — Redis for hot data (dashboard stats, vendor listings)
4. **Rate limiting** — Redis-backed rate limiter per API key / user
5. **Horizontal scaling** — Stateless API servers behind a load balancer
6. **Database optimization** — Read replicas, query optimization, indexing
7. **Bulk operations** — Batch inserts for guest imports, async job queues
8. **Load testing** — Use `k6` or `wrk` to benchmark endpoints

## 📋 How to Help

### When Writing Rust Code:
1. **Always explain syntax** - Compare to Go/PHP equivalents when helpful
2. **Show best practices** - Reference known repositories (tokio, actix examples)
3. **Explain ownership/borrowing** - This is new concept from Go/PHP
4. **Add comments** - Explain what each part does
5. **Show error handling patterns** - Result<T, E> vs Go's (val, err)
6. **Concurrency patterns** - Explain Arc, Mutex, channels vs Go's goroutines/channels

### Example Format for Rust Code:
```rust
// In Go, you'd write: func getData() (string, error)
// In Rust, we use Result<T, E> which is similar but type-safe

async fn get_data() -> Result<String, AppError> {
    // ? operator is like Go's "if err != nil { return err }"
    let data = fetch_something().await?;

    // In Rust, last expression without ; is the return value
    // Similar to Go's explicit return, but more concise
    Ok(data)
}
```

### When Discussing Architecture:
1. **Explain WHY** - Not just what, but why this pattern
2. **Compare alternatives** - Show trade-offs (REST vs gRPC, SQL vs NoSQL)
3. **Link to resources** - Rust book, actix docs, tonic (gRPC), etc.
4. **Suggest learning order** - What to learn first
5. **Scalability considerations** - How this decision affects high concurrency

### When Helping with DevOps:
1. **Explain each service** - What it does, why separate
2. **Show connection between services** - How they communicate (REST vs gRPC)
3. **Security considerations** - What to watch out for
4. **Cost breakdown** - Keep it free/cheap for development

## 🎭 User Roles in the App

| Role | Users | Access |
|------|-------|--------|
| **admin** | Event organizers | Full CRUD everything |
| **family** | Close family members | Read-only, prices hidden (toggle) |
| **guest** | Invited guests | View invitation, submit RSVP |

## 📁 Project Structure

```
wedding-planner/
├── CLAUDE.md           # This file - context for Claude
├── api/                # Rust backend (REST + gRPC)
├── web/                # Vue 3 frontend
├── crawler/            # Rust web crawler for wedding vendor info
├── proto/              # Protocol Buffer definitions (gRPC)
├── docs/               # Documentation
├── data/               # Seed data
├── deploy/             # Docker & deployment configs
└── .github/workflows/  # CI/CD pipelines
```

## 🔧 Current Development Phase

**Phase:** Project Setup & Foundation
**Next:** Build Budget API, implement auth, add vendor management

## 📊 Data Context

Vendor pricing data for weddings:
- Wedding organizer packages
- Hotel venue packages
- Sub-vendor categories: Catering, Decoration, MUA, Photo, Entertainment, MC

This data should be **seeded into database** for vendor selection and comparison.

## ✨ Special Requests

1. **Rust Teaching Mode** - Explain syntax, compare to Go/PHP
2. **Best Practices** - Use patterns from well-known Rust projects
3. **DevOps Learning** - Explain infrastructure concepts
4. **gRPC Learning** - When and how to use RPC vs REST
5. **High Concurrency** - Design for scale, benchmark, optimize
6. **Wedding Crawler** - Automation to find wedding vendor events
7. **Mobile-First** - Guest invitation page via WhatsApp
8. **SaaS-Ready** - Multi-tenant architecture for monetization

## 📚 Learning Resources to Reference

### Rust:
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Actix-web: https://actix.rs/docs/
- Tokio (async): https://tokio.rs/tokio/tutorial
- SeaORM: https://www.sea-ql.org/SeaORM/

### gRPC:
- Tonic (Rust gRPC): https://github.com/hyperium/tonic
- Protocol Buffers: https://protobuf.dev/
- gRPC Official: https://grpc.io/docs/

### NoSQL:
- Redis Docs: https://redis.io/docs/
- MongoDB Rust Driver: https://www.mongodb.com/docs/drivers/rust/current/

### Reference Repositories:
- actix/examples: https://github.com/actix/examples
- realworld-actix-web: https://github.com/fairingrey/actix-realworld-example-app
- zero2prod: https://github.com/LukeMathWalker/zero-to-production

### DevOps:
- Docker Docs: https://docs.docker.com/
- GitHub Actions: https://docs.github.com/en/actions

## 🚀 Common Commands

```bash
# Rust
cargo build              # Compile
cargo run -p api         # Run API server
cargo run -p crawler     # Run crawler
cargo test               # Test
cargo clippy             # Linter
cargo fmt                # Format code

# Docker
docker compose up -d     # Start services
docker compose logs -f   # View logs

# Git (conventional commits)
git add . && git commit -m "feat: description" && git push
```

## 💡 Tips for Claude

1. **Be thorough** - Developer is learning, more detail is better
2. **Be practical** - This is a real product, not just exercise
3. **Be encouraging** - Rust has steep learning curve
4. **Be proactive** - Suggest improvements and alternatives
5. **Think scale** - Consider high concurrency in every design decision
6. **Remember context** - Reference this file for project details

---

*Last Updated: February 7, 2026*
