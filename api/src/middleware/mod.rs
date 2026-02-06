//! HTTP middleware (authentication, logging, CORS, etc).
//!
//! In Go, middleware wraps handlers: func(next http.Handler) http.Handler.
//! In PHP/Laravel, middleware is defined in app/Http/Middleware/.
//! In Rust with Actix, middleware implements the Transform trait,
//! but Actix also provides helper macros and built-in middleware.
