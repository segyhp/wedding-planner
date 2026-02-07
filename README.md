# 💒 Wedding Planner

> A high-concurrency, full-stack wedding planning platform built with **Rust** and **Vue 3** — designed as a SaaS product for event organizers and couples.

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3.5+-green?logo=vue.js)](https://vuejs.org/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16+-blue?logo=postgresql)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

---

## 🎯 About This Project

A wedding planning platform that helps couples and event organizers manage their entire wedding — budget, vendors, guests, and invitations — all in one place.

### Key Features

- 💰 **Budget Management** — Track expenses with ownership split between parties
- 🏪 **Vendor Management** — Compare and book wedding vendors
- 👥 **Guest Management** — RSVP tracking with custom fields and bulk import
- 📱 **Mobile Invitation** — WhatsApp-friendly invitation page
- 📊 **Dashboard** — Real-time wedding planning overview
- 🔍 **Wedding Crawler** — Auto-discover wedding vendor events
- ⚡ **High Concurrency** — Built for scale with async Rust and connection pooling

---

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Backend** | Rust + Actix-web | REST API (client-facing) |
| **Internal RPC** | gRPC (Tonic) | Service-to-service communication |
| **Frontend** | Vue 3 + Vite + TypeScript | SPA |
| **Database** | PostgreSQL 16 | Primary data store |
| **Cache** | Redis | Session, caching, rate limiting |
| **NoSQL** | MongoDB (optional) | Flexible vendor data, event logs |
| **Auth** | JWT + argon2 | Authentication |

### API Strategy

| Scenario | Protocol | Why |
|----------|----------|-----|
| Frontend ↔ Backend | REST | Browser-friendly, JSON |
| API ↔ Crawler | gRPC | Fast, typed, internal |
| API ↔ Notifications | gRPC | Low-latency, streaming |
| Third-party integrations | REST | Universal compatibility |

### Hosting Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Cloudflare │────▶│   Vercel    │     │  Supabase   │
│  (DNS/CDN)  │     │  (Vue SPA)  │     │ (PostgreSQL)│
└─────────────┘     └──────┬──────┘     └──────▲──────┘
                           │                    │
                           ▼                    │
                    ┌─────────────┐             │
                    │   Railway   │─────────────┘
                    │ (Rust API)  │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │   Upstash   │
                    │   (Redis)   │
                    └─────────────┘
```

**Monthly Cost: $0-5** using free tiers

---

## 📁 Project Structure

```
wedding-planner/
├── README.md
├── CLAUDE.md                 # AI context for Claude
├── .github/
│   └── workflows/
│       ├── ci.yml            # Main CI/CD pipeline
│       └── crawler.yml       # Scheduled crawler
│
├── api/                      # 🦀 Rust Backend
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs
│       ├── config/           # Configuration
│       ├── handlers/         # Request handlers
│       ├── models/           # Database models
│       ├── routes/           # API routes
│       ├── services/         # Business logic
│       └── middleware/       # Auth, logging, etc.
│
├── web/                      # 💚 Vue Frontend
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── components/       # Reusable components
│       ├── pages/            # Route pages
│       ├── stores/           # Pinia stores
│       ├── composables/      # Vue composables
│       └── api/              # API client
│
├── crawler/                  # 🕷️ Wedding Vendor Crawler
│   ├── Cargo.toml
│   └── src/
│
├── proto/                    # 📡 Protocol Buffer Definitions
│   └── *.proto
│
├── migrations/               # 📦 Database Migrations
│   ├── 001_initial.up.sql
│   └── 001_initial.down.sql
│
├── data/                     # 📊 Seed Data
│   └── vendors/              # Vendor info
│
└── docs/                     # 📚 Documentation
    ├── 01_PROJECT_OVERVIEW.md
    ├── 02_SYSTEM_ARCHITECTURE.md
    ├── 03_DATABASE_DESIGN.md
    ├── 04_API_SPECIFICATION.md
    ├── 05_RUST_LEARNING_GUIDE.md
    ├── 06_VENDOR_COMPARISON.md
    ├── 07_DEVOPS_HOSTING_GUIDE.md
    ├── 08_IMPLEMENTATION_PLAN.md
    └── 09_CRAWLER_DESIGN.md
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.85+
- [Node.js](https://nodejs.org/) 20+
- [Docker](https://www.docker.com/) (for local development)
- [PostgreSQL](https://www.postgresql.org/) 16+ (or use Docker)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/your-username/wedding-planner.git
cd wedding-planner

# Start local services (PostgreSQL + Redis)
docker compose up -d

# Setup API
cd api
cp .env.example .env
cargo run

# Setup Web (in another terminal)
cd web
cp .env.example .env
npm install
npm run dev
```

### Environment Variables

**API (.env)**
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/wedding
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-key
RUST_LOG=info
```

**Web (.env)**
```env
VITE_API_URL=http://localhost:8080/v1
```

---

## 👥 User Roles

| Role | Users | Permissions |
|------|-------|-------------|
| `admin` | Event organizers | Full CRUD on all data |
| `family` | Close family members | Read-only, prices hidden (toggleable) |
| `guest` | Invited guests | View invitation, submit RSVP |

---

## 📊 Database Schema

Key entities with ownership tracking:

```sql
-- Ownership split between parties
CREATE TYPE belongs_to AS ENUM ('party_a', 'party_b', 'both');

-- Guests table with ownership
CREATE TABLE guests (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    belongs_to belongs_to DEFAULT 'both',
    -- ...
);

-- Budget items with payment responsibility
CREATE TABLE budget_items (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    belongs_to belongs_to DEFAULT 'both',
    -- ...
);
```

See [03_DATABASE_DESIGN.md](docs/03_DATABASE_DESIGN.md) for complete schema.

---

## 🔌 API Endpoints

### Authentication
```
POST /v1/auth/register    # Register new user
POST /v1/auth/login       # Login
POST /v1/auth/refresh     # Refresh token
POST /v1/auth/logout      # Logout
```

### Budget
```
GET    /v1/projects/:id/budget/summary     # Budget overview
GET    /v1/projects/:id/budget/categories  # List categories
POST   /v1/projects/:id/budget/items       # Create item
POST   /v1/projects/:id/budget/items/:id/payments  # Record payment
```

### Vendors
```
GET    /v1/projects/:id/vendors           # List vendors
POST   /v1/projects/:id/vendors           # Add vendor
POST   /v1/projects/:id/vendors/:id/book  # Book vendor
GET    /v1/vendors/search                 # Search all vendors
```

### Guests
```
GET    /v1/projects/:id/guests            # List guests
POST   /v1/projects/:id/guests            # Add guest
POST   /v1/projects/:id/guests/import     # Bulk import
PUT    /v1/projects/:id/guests/:id/rsvp   # Update RSVP
```

See [04_API_SPECIFICATION.md](docs/04_API_SPECIFICATION.md) for complete API docs.

---

## ⚡ High Concurrency Design

This project is designed for scale:

- **Async runtime** — Tokio + Actix-web for non-blocking I/O
- **Connection pooling** — `deadpool` / `bb8` for PostgreSQL and Redis
- **Caching** — Redis for hot data (dashboard stats, vendor listings)
- **Rate limiting** — Redis-backed per-user / per-API-key rate limiter
- **Stateless API** — Horizontal scaling behind load balancer
- **Bulk operations** — Batch inserts, async job queues
- **gRPC internal** — Low-latency service-to-service communication

---

## 🧪 Testing

```bash
# API tests
cd api
cargo test

# With coverage
cargo tarpaulin --out Html

# Web tests
cd web
npm run test
npm run test:e2e

# Load testing
k6 run tests/load/budget-api.js
```

---

## 🚢 Deployment

### Automatic (GitHub Actions)

Push to `main` branch triggers:
1. Run tests (Rust + Vue)
2. Build Docker image
3. Deploy API to Railway
4. Deploy Web to Vercel
5. Run database migrations

### Manual

```bash
# Build API
cd api
cargo build --release

# Build Web
cd web
npm run build
```

See [07_DEVOPS_HOSTING_GUIDE.md](docs/07_DEVOPS_HOSTING_GUIDE.md) for detailed deployment instructions.

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [CLAUDE.md](CLAUDE.md) | Context for Claude AI assistance |
| [01_PROJECT_OVERVIEW.md](docs/01_PROJECT_OVERVIEW.md) | Project summary and goals |
| [02_SYSTEM_ARCHITECTURE.md](docs/02_SYSTEM_ARCHITECTURE.md) | Architecture diagrams |
| [03_DATABASE_DESIGN.md](docs/03_DATABASE_DESIGN.md) | Complete SQL schema |
| [04_API_SPECIFICATION.md](docs/04_API_SPECIFICATION.md) | REST API documentation |
| [05_RUST_LEARNING_GUIDE.md](docs/05_RUST_LEARNING_GUIDE.md) | Rust tutorial (Go/PHP comparison) |
| [06_VENDOR_COMPARISON.md](docs/06_VENDOR_COMPARISON.md) | Vendor data and pricing |
| [07_DEVOPS_HOSTING_GUIDE.md](docs/07_DEVOPS_HOSTING_GUIDE.md) | CI/CD and hosting setup |
| [08_IMPLEMENTATION_PLAN.md](docs/08_IMPLEMENTATION_PLAN.md) | Development timeline |
| [09_CRAWLER_DESIGN.md](docs/09_CRAWLER_DESIGN.md) | Wedding vendor crawler |

---

## 🎓 Learning Resources

This project covers multiple learning goals:

### Rust
1. **[The Rust Book](https://doc.rust-lang.org/book/)** — Official tutorial
2. **[Rustlings](https://github.com/rust-lang/rustlings)** — Practice exercises
3. **[Zero to Production](https://www.zero2prod.com/)** — Building APIs in Rust
4. **[Actix Web Examples](https://github.com/actix/examples)** — Framework examples

### gRPC
1. **[Tonic](https://github.com/hyperium/tonic)** — Rust gRPC framework
2. **[Protocol Buffers](https://protobuf.dev/)** — Serialization format
3. **[gRPC Docs](https://grpc.io/docs/)** — Official documentation

### NoSQL
1. **[Redis University](https://university.redis.io/)** — Free courses
2. **[MongoDB Rust Driver](https://www.mongodb.com/docs/drivers/rust/current/)** — Official docs

---

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Open issues for bugs or feature ideas
- Submit PRs for improvements
- Suggest wedding planning features

---

## 📄 License

MIT License — feel free to use this as a template for your own wedding planner or event management platform!

---

<p align="center">
  Made with ❤️ and 🦀
</p>
