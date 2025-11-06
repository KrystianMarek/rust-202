# Documentation Index

Welcome to the rust-202 documentation. This directory contains comprehensive guides for the project.

---

## 📚 Documentation Files

### Core Documentation

1. **[PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)** - Start here!
   - What is rust-202
   - Workspace structure (2 crates)
   - All 7 library modules
   - 21 REST + 3 gRPC endpoints
   - Quick start guide

2. **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical deep-dive
   - Module organization
   - Design principles
   - Data flow
   - Testing strategy
   - Deployment

3. **[ASYNC_MODULE_SUMMARY.md](ASYNC_MODULE_SUMMARY.md)** - Async patterns guide
   - Advanced async/await
   - Streams and futures
   - Concurrency patterns
   - Comparisons to Python/Go/C

4. **[DSL_GUIDE.md](DSL_GUIDE.md)** 🆕 - Python embedding guide
   - PyO3 integration
   - Safe sandboxing
   - Script execution
   - Data transformation
   - Configuration DSL

### Project Management

5. **[CHANGELOG.md](CHANGELOG.md)** - Version history
   - Release notes
   - Feature additions
   - Breaking changes

6. **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
   - Coding standards
   - PR process
   - Testing requirements
   - Documentation guidelines

---

## Quick Navigation

### I want to...

**Learn Rust patterns**:
→ Start with [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
→ Browse `rust-202-lib/src/` source code
→ Run `cargo run -p rust-202 --example quickstart`

**Understand the architecture**:
→ Read [ARCHITECTURE.md](ARCHITECTURE.md)
→ See module organization and design decisions

**Use async Rust**:
→ Read [ASYNC_MODULE_SUMMARY.md](ASYNC_MODULE_SUMMARY.md)
→ Explore `rust-202-lib/src/async/`
→ Run `cargo run -p rust-202 --example async_demo`

**Embed Python in Rust**:
→ Read [DSL_GUIDE.md](DSL_GUIDE.md)
→ Explore `rust-202-lib/src/dsl/python/`
→ Try `POST /dsl/sandbox` endpoint

**Build the web API**:
→ See `../rust-202-web/README.md`
→ Run `cargo run -p rust-202-web --no-default-features`
→ Visit http://localhost:3000/docs

**Contribute**:
→ Read [CONTRIBUTING.md](CONTRIBUTING.md)
→ Check [CHANGELOG.md](CHANGELOG.md) for current state
→ Open a PR!

---

## External Documentation

- **Main README**: `../README.md` - Project homepage
- **Library README**: Inline rustdoc - `cargo doc -p rust-202 --open`
- **Web App README**: `../rust-202-web/README.md` - API server guide
- **Web App Quick Start**: `../rust-202-web/QUICK_START.md` - Getting started

---

## Documentation Philosophy

All documentation follows these principles:

1. **Current State Focused** - No migration/refactoring history
2. **Practical Examples** - Working code snippets
3. **Comparisons** - Rust vs Python/Go/C
4. **No Duplication** - Each doc has a clear purpose
5. **Up-to-Date** - Reflects actual code

---

## File Sizes

```
PROJECT_OVERVIEW.md      ~8KB    High-level overview (updated)
ARCHITECTURE.md          ~10KB   Technical details
ASYNC_MODULE_SUMMARY.md  ~9KB    Async patterns guide
DSL_GUIDE.md            ~7KB    Python DSL guide (new!)
CHANGELOG.md             ~3KB    Version history
CONTRIBUTING.md          ~5KB    Contribution guidelines
```

**Total**: ~42KB of focused, non-redundant documentation

---

## Maintenance

### Adding Documentation

When adding new features:
1. Update inline rustdoc in source files
2. Add examples to relevant module summary
3. Update CHANGELOG.md
4. Keep docs focused on current state

### Removing Outdated Docs

This directory has been cleaned of:
- ❌ Migration guides (outdated)
- ❌ Implementation details (transient)
- ❌ Status updates (superseded)
- ❌ Duplicate information

---

## Quick Reference

| Question | Document |
|----------|----------|
| What is this project? | PROJECT_OVERVIEW.md |
| How is it organized? | ARCHITECTURE.md |
| How do I use async? | ASYNC_MODULE_SUMMARY.md |
| What changed? | CHANGELOG.md |
| How can I help? | CONTRIBUTING.md |

---

**Last Updated**: October 31, 2025
**Total Docs**: 5 markdown files (+ this index)
**Status**: Consolidated and current

