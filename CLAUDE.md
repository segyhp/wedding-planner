# CLAUDE.md - Segy & Azizah Wedding Planner Project

## 🎯 Project Context

This is a **personal learning project** for Segy to:
1. Learn **Rust** programming (first time, coming from Go/PHP background)
2. Learn **DevOps/CI-CD** practices with multiple servers
3. Build a functional **wedding planner** for actual use (Wedding: Sept 19-20, 2026)

## 👤 About the Developer

- **Name:** Segy (Groom)
- **Experience:** Go, PHP (bearFramework - legacy code), wants to learn modern tech
- **Learning Style:** Challenge mode - learn by doing real projects
- **IDE:** RustRover
- **Subscription:** Claude Pro

## 🏗️ Tech Stack

| Layer | Technology | Status |
|-------|------------|--------|
| Backend | **Rust + Actix-web** | Learning |
| Frontend | **Vue 3 + Vite** (SPA) | Familiar territory |
| Database | **PostgreSQL** | Experienced |
| Cache | Redis | Familiar |
| Hosting | Multiple servers (free tier) | Learning |
| CI/CD | GitHub Actions | Learning |

## 📋 How to Help Segy

### When Writing Rust Code:
1. **Always explain syntax** - Compare to Go/PHP equivalents when helpful
2. **Show best practices** - Reference known repositories (tokio, actix examples)
3. **Explain ownership/borrowing** - This is new concept from Go/PHP
4. **Add comments** - Explain what each part does
5. **Show error handling patterns** - Result<T, E> vs Go's (val, err)

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
2. **Compare alternatives** - Show trade-offs
3. **Link to resources** - Rust book, actix docs, etc.
4. **Suggest learning order** - What to learn first

### When Helping with DevOps:
1. **Explain each service** - What it does, why separate
2. **Show connection between services** - How they communicate
3. **Security considerations** - What to watch out for
4. **Cost breakdown** - Keep it free/cheap

## 🎭 User Roles in the App

| Role | Users | Access |
|------|-------|--------|
| **admin** | Segy, Azizah | Full CRUD everything |
| **family** | Parents, siblings | Read-only, prices hidden (toggle) |
| **guest** | Invited guests | View invitation, submit RSVP |

## 📁 Project Structure

```
wedding-planner/
├── CLAUDE.md           # This file - context for Claude
├── api/                # Rust backend
├── web/                # Vue 3 frontend
├── crawler/            # Rust web crawler for Semarang wedding info
├── docs/               # Documentation
├── data/               # Seed data from PDFs
├── deploy/             # Docker & deployment configs
└── .github/workflows/  # CI/CD pipelines
```

## 🔧 Current Development Phase

**Phase:** Documentation Review & Planning
**Next:** Setup Rust project, parse PDF data, build Budget API

## 📊 Data Context

PDF files contain vendor pricing data for Semarang weddings:
- Dalang Wedding Organizer (4 packages)
- Hotel Aruss (4 packages)
- Padma Hotel (3 packages)
- Each has sub-vendors: Catering, Decoration, MUA, Photo, Entertainment, MC

This data should be **seeded into database** for vendor selection.

## ✨ Special Requests

1. **Rust Teaching Mode** - Explain syntax, compare to Go/PHP
2. **Best Practices** - Use patterns from well-known Rust projects
3. **DevOps Learning** - Explain infrastructure concepts
4. **Wedding Crawler** - Automation to find Semarang wedding events
5. **Mobile-First** - Guest invitation page via WhatsApp

## 📚 Learning Resources to Reference

### Rust:
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Actix-web: https://actix.rs/docs/
- Tokio (async): https://tokio.rs/tokio/tutorial
- SeaORM: https://www.sea-ql.org/SeaORM/

### Reference Repositories:
- actix/examples: https://github.com/actix/examples
- realworld-actix-web: https://github.com/fairingrey/actix-realworld-example-app
- zero2prod: https://github.com/LukeMathWalker/zero-to-production

### DevOps:
- Docker Docs: https://docs.docker.com/
- GitHub Actions: https://docs.github.com/en/actions

## 🚀 Commands Segy Uses

```bash
# Rust
cargo new project_name    # Create new project
cargo build              # Compile
cargo run                # Run
cargo test               # Test
cargo clippy             # Linter
cargo fmt                # Format code

# Docker
docker compose up -d     # Start services
docker compose logs -f   # View logs

# Git
git add . && git commit -m "msg" && git push
```

## 💡 Tips for Claude

1. **Be thorough** - Segy is learning, more detail is better
2. **Be practical** - This is a real wedding, not just exercise
3. **Be encouraging** - Rust has steep learning curve
4. **Be proactive** - Suggest improvements and alternatives
5. **Remember context** - Reference this file for project details

---

*Last Updated: January 31, 2026*
*Project Owner: Segy*
*Wedding Date: September 19/20, 2026*
