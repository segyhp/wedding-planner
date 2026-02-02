# 💒 Segy & Azizah Wedding Planner

> A full-stack wedding planning application built with **Rust** and **Vue 3** — combining a real wedding planning tool with a learning project for modern web development.

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3.4+-green?logo=vue.js)](https://vuejs.org/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16+-blue?logo=postgresql)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

---

## 🎯 About This Project

This wedding planner is being built for **Segy & Azizah's wedding** on **September 19-20, 2026** in Semarang, Indonesia. It serves dual purposes:

1. **Practical Tool** — Manage budget, vendors, guests, and timeline for the actual wedding
2. **Learning Project** — First Rust project for a developer coming from Go/PHP background

### Key Features

- 💰 **Budget Management** — Track expenses with ownership (Segy/Azizah/Both)
- 🏪 **Vendor Management** — Compare and book wedding vendors
- 👥 **Guest Management** — RSVP tracking with custom fields
- 📱 **Mobile Invitation** — WhatsApp-friendly invitation page
- 📊 **Dashboard** — Real-time wedding planning overview
- 🔍 **Wedding Crawler** — Auto-discover Semarang wedding events

---

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Backend** | Rust + Actix-web | REST API |
| **Frontend** | Vue 3 + Vite + TypeScript | SPA |
| **Database** | PostgreSQL 16 | Primary data store |
| **Cache** | Redis (Upstash) | Session & caching |
| **Auth** | JWT + argon2 | Authentication |

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
                          ▼
                   ┌─────────────┐
                   │   Upstash   │
                   │   (Redis)   │
                   └─────────────┘
```

**Monthly Cost: $0-5** using free tiers

---

## 📁 Project Structure

```
segy-azizah-wedding/
├── README.md
├── CLAUDE.md                 # AI context for Claude Pro
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
├── crawler/                  # 🕷️ Wedding Event Crawler
│   ├── Cargo.toml
│   └── src/
│
├── migrations/               # 📦 Database Migrations
│   ├── 001_initial.up.sql
│   └── 001_initial.down.sql
│
├── data/                     # 📊 Seed Data
│   └── vendors/              # Vendor info from PDFs
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

- [Rust](https://rustup.rs/) 1.75+
- [Node.js](https://nodejs.org/) 20+
- [Docker](https://www.docker.com/) (for local development)
- [PostgreSQL](https://www.postgresql.org/) 16+ (or use Docker)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/segy/segy-azizah-wedding.git
cd segy-azizah-wedding

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
| `admin` | Segy, Azizah | Full CRUD on all data |
| `family` | Parents, siblings | Read-only, prices hidden (toggleable) |
| `guest` | Invited guests | View invitation, submit RSVP |

---

## 📊 Database Schema

Key entities with ownership tracking:

```sql
-- Guest ownership
CREATE TYPE belongs_to AS ENUM ('segy', 'azizah', 'both');

-- Guests table with ownership
CREATE TABLE guests (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    belongs_to belongs_to DEFAULT 'both',  -- Whose guest?
    -- ...
);

-- Budget items with payment responsibility
CREATE TABLE budget_items (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    belongs_to belongs_to DEFAULT 'both',  -- Who pays?
    -- ...
);
```

See [03_DATABASE_DESIGN.md](docs/03_DATABASE_DESIGN_v3.md) for complete schema.

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
| [01_PROJECT_OVERVIEW.md](docs/01_PROJECT_OVERVIEW_v2.md) | Project summary and goals |
| [02_SYSTEM_ARCHITECTURE.md](docs/02_SYSTEM_ARCHITECTURE.md) | Architecture diagrams |
| [03_DATABASE_DESIGN.md](docs/03_DATABASE_DESIGN_v3.md) | Complete SQL schema |
| [04_API_SPECIFICATION.md](docs/04_API_SPECIFICATION.md) | REST API documentation |
| [05_RUST_LEARNING_GUIDE.md](docs/05_RUST_LEARNING_GUIDE.md) | Rust tutorial (Go/PHP comparison) |
| [06_VENDOR_COMPARISON.md](docs/06_VENDOR_COMPARISON.md) | Vendor data and pricing |
| [07_DEVOPS_HOSTING_GUIDE.md](docs/07_DEVOPS_HOSTING_GUIDE.md) | CI/CD and hosting setup |
| [08_IMPLEMENTATION_PLAN.md](docs/08_IMPLEMENTATION_PLAN.md) | Development timeline |
| [09_CRAWLER_DESIGN.md](docs/09_CRAWLER_DESIGN.md) | Wedding event crawler |

---

## 🎓 Learning Resources

This project is designed as a Rust learning experience. Recommended resources:

1. **[The Rust Book](https://doc.rust-lang.org/book/)** — Official tutorial
2. **[Rustlings](https://github.com/rust-lang/rustlings)** — Practice exercises
3. **[Zero to Production](https://www.zero2prod.com/)** — Building APIs in Rust
4. **[Actix Web Examples](https://github.com/actix/examples)** — Framework examples

See [05_RUST_LEARNING_GUIDE.md](docs/05_RUST_LEARNING_GUIDE.md) for concepts mapped to Go/PHP.

---

## 🤝 Contributing

This is a personal project, but suggestions are welcome! Feel free to:

- Open issues for bugs or feature ideas
- Submit PRs for improvements
- Share wedding planning tips 😄

---

## 📄 License

MIT License — feel free to use this as a template for your own wedding planner!

---

## 💑 About Us

**Segy & Azizah** — Getting married on September 19-20, 2026 in Semarang, Central Java, Indonesia.

Building this app together as we plan our wedding! 🎉

---

<p align="center">
  Made with ❤️ in Tokyo, Japan
  <br>
  <sub>Wedding Date: September 19-20, 2026</sub>
</p>