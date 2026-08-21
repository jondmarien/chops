# Chops Linux Port: High-Level Overview

## The Problem

Chops is a brilliant macOS-only app for organizing AI agent skills, but it locks out Windows and Linux developers who use Cursor, Windsurf, Claude, and other cross-platform IDEs. A Linux port unlocks an untapped market segment while validating the product's cross-platform potential.

## The Solution

**Port Chops to Linux (Arch Linux MVP) using Tauri**, a lightweight Rust framework that compiles to a ~7 MB native binary with zero Electron bloat. Extract 60% of Chops' business logic (skill discovery, parsing, persistence) from Swift to Rust; rebuild 40% of the UI in Svelte. **Result: A 6–8 week project delivering a production-ready Linux app with feature parity to the macOS version.**

---

## Why Tauri (Not Electron or Qt)

| Criteria | Tauri | Electron | Qt |
|----------|-------|----------|-----|
| **Bundle Size** | ~7 MB | ~150 MB | ~20+ MB |
| **Memory Footprint** | ~50 MB | ~200+ MB | ~80+ MB |
| **Startup Time** | <500 ms | 1–2 s | <300 ms |
| **Language Match** | Rust (native) | JS/Node | C++ |
| **Web Tech** | Yes (Svelte) | Yes (React/Vue) | No |
| **Cross-Platform** | Windows, macOS, Linux | Windows, macOS, Linux | Windows, macOS, Linux |
| **Distribution Ease** | AppImage, native installers | NSIS, DMG | Complex |
| **Learning Curve** | Medium (Rust) | Low (JS) | Steep (C++) |

**Tauri wins** because it's fast, lean, and Rust makes it easy to port the business logic directly from Swift.

---

## What Gets Reused (60%)

### Core Logic — No UI, Pure Algorithms
1. **SkillScanner** — Probe `~/.claude/`, `~/.cursor/`, etc. for skills
2. **SkillParser** — Extract metadata from `.md`, `.mdc`, `.toml` files
3. **FrontmatterParser** — Parse YAML headers in Markdown
4. **ToolSource** — Tool detection, path resolution
5. **State Management** — AppState logic (filtering, searching, selection)
6. **Models** — Skill, Collection, ToolSource domain objects

### Process
1. Read original Swift files (SkillScanner.swift, SkillParser.swift, etc.)
2. Convert Swift types → Rust structs
3. Convert Swift `FileManager` API calls → Rust `std::fs` + `walkdir` crate
4. Port frontmatter regex parsing as-is
5. Adapt tool paths for Linux (XDG Base Directory Spec)

**Expected code reuse:** ~1,500–1,800 lines of Rust derived from Swift.

---

## What Gets Rebuilt (40%)

### UI Layer (SwiftUI → Svelte)
- Three-column layout (sidebar, skill list, detail pane)
- Skill editor with syntax highlighting (CodeMirror 6)
- Collection management
- Search & filtering
- Settings/preferences
- Responsive design for different window sizes

**Estimated lines:** ~2,000–2,500 TypeScript + HTML + CSS

### Persistence (SwiftData → SQLite)
- Database schema (mirrored from SwiftData models)
- CRUD operations for skills, collections, settings
- Transaction handling
- Schema migrations

**Estimated lines:** ~500–700 SQL + Rust

### File Watching (FSEvents → notify-rs)
- Rust `notify` crate for inotify (Linux), FSEvents (macOS), ReadDirectoryChangesW (Windows)
- Debouncing (500ms to batch rapid changes)
- Rescan trigger when files change

**Estimated lines:** ~200 Rust

### IPC / Tauri Commands
- Expose Rust functions as Tauri commands (callable from frontend)
- Commands: `scan_all()`, `save_skill()`, `create_skill()`, `add_to_collection()`, etc.
- Error handling + response serialization

**Estimated lines:** ~400 Rust

---

## Architecture at a Glance

```
┌─────────────────────────────────────────────────────┐
│           Svelte Frontend (TypeScript)              │
│  ┌──────────────────────────────────────────────┐  │
│  │ Three-Column Layout                          │  │
│  │ ┌─────────┬──────────────┬─────────────────┐ │  │
│  │ │ Sidebar │  Skill List  │  Skill Editor   │ │  │
│  │ │(Tools,  │ (Search,     │ (CodeMirror,    │ │  │
│  │ │Collect.)│  Filter)     │  Save, Publish) │ │  │
│  │ └─────────┴──────────────┴─────────────────┘ │  │
│  └──────────────────────────────────────────────┘  │
│              Svelte Stores (State)                  │
└────────────────────┬─────────────────────────────────┘
                     │ Tauri IPC (JSON-RPC)
┌────────────────────▼─────────────────────────────────┐
│         Rust Backend (Tauri Commands)                │
│  ┌──────────────────────────────────────────────┐   │
│  │ scanner.rs      │ Save/Load Skills           │   │
│  │ parser.rs       │ Parse Frontmatter          │   │
│  │ watcher.rs      │ Monitor Filesystem         │   │
│  │ db.rs           │ SQLite Queries             │   │
│  │ tools.rs        │ Tool Detection             │   │
│  │ commands.rs     │ Tauri IPC Handlers         │   │
│  └──────────────────────────────────────────────┘   │
│              SQLite Database                        │
│  ┌──────────────────────────────────────────────┐   │
│  │ skills | collections | skill_collections    │   │
│  │ settings | remote_servers (future)           │   │
│  └──────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────┘
           ▲
           │ Read/Write
           │
    ┌──────▼──────┐
    │   Filesystem │
    │ ~/.claude/   │
    │ ~/.cursor/   │
    │ ~/.codex/    │
    │ ~/.agents/   │
    └─────────────┘
```

---

## Key Features (MVP)

### 1. **Skill Discovery & Organization**
- Auto-scan `~/.claude/skills/`, `~/.cursor/skills/`, etc.
- Display as filterable list (by tool, kind: skill/agent/rule)
- Show install count (how many tools a skill is available in)
- Collections for organizing without modifying files

### 2. **Skill Editing**
- In-app editor with syntax highlighting
- Edit YAML frontmatter (name, description, tags)
- Save to disk with Ctrl+S
- Real-time validation (check for malformed YAML)

### 3. **Search & Filter**
- Full-text search across skill names, descriptions, content
- Filter by tool (Claude, Cursor, Windsurf, etc.)
- Filter by kind (skill, agent, rule)
- Sort by name, modified date, size

### 4. **File Watching**
- Real-time sync with disk
- If you edit a skill in Cursor, Chops detects and refreshes
- Non-blocking (doesn't freeze the UI)

### 5. **Collections**
- Group skills without modifying source files
- Create, rename, delete collections
- Add/remove skills from collections
- Filter by collection

### 6. **Settings**
- Custom scan paths (for project-specific skills)
- Tool configuration (which tools to show/hide)
- Preferences (theme, sort order, auto-refresh interval)
- Show/hide debug logging

---

## Why This Works

### 1. **Swift → Rust Translation is Mechanical**
Most of Chops' complexity is in the UI (SwiftUI), not the logic. Porting SkillScanner is straightforward:
- `FileManager` → `std::fs` / `walkdir`
- `Array<Skill>` → `Vec<Skill>`
- Regex parsing → same regex engine in Rust

### 2. **Tauri Is Purpose-Built for This**
It's designed for Electron-alternative shipping. Tauri handles:
- Cross-platform compilation
- Native file dialogs, menus, tray icons
- Seamless frontend-backend communication (IPC)
- Auto-updates (optional)

### 3. **60% Code Reuse Saves Months**
By extracting the business logic, you're not building from scratch. You're building the "glue layer" (UI + IPC + DB).

### 4. **SQLite is Stable & Proven**
SwiftData is elegant but proprietary to Apple. SQLite is ubiquitous, performant, and ships with Tauri. Migrations are straightforward with sqlx.

### 5. **Linux Users Are Underserved**
Cursor, Windsurf, and Claude run on Linux. No existing solution (that I'm aware of) organizes skills across these tools on Linux. **First-mover advantage.**

---

## Success Metrics

### Technical
- ✅ Binary size < 10 MB (compressed)
- ✅ Startup time < 1 second
- ✅ Scan 500+ skills in < 2 seconds
- ✅ 100+ FPS on list scroll with 500 skills
- ✅ Memory footprint < 150 MB idle

### Functional
- ✅ Supports Claude Code, Cursor, Windsurf, Codex, Amp on Linux
- ✅ Search finds skills in < 100 ms
- ✅ File watcher detects changes within 1 second
- ✅ Save works reliably (atomic writes)
- ✅ Collections persist across sessions

### User-Facing
- ✅ Runs on Arch Linux (x86_64)
- ✅ Installable via AUR or AppImage
- ✅ Works offline
- ✅ Clear error messages
- ✅ Builds from source without issues

---

## Timeline at a Glance

```
Week 1-2    [Rust Backend]
  - Port SkillScanner, SkillParser, ToolSource
  - Set up SQLite schema
  - Build Tauri commands

Week 2-3    [IPC & File Watching]
  - Tauri command handlers
  - notify-rs integration
  - State synchronization

Week 3-5    [Svelte Frontend]
  - Three-column layout
  - Skill list & search
  - Skill editor with CodeMirror
  - Collections management

Week 5-6    [Polish & Testing]
  - Error handling
  - Edge cases
  - Arch Linux testing
  - Settings & preferences

Week 6-7    [Packaging & Release]
  - AppImage build
  - AUR PKGBUILD
  - CI/CD pipeline
  - Documentation

Week 7-8    [Contingency & Iteration]
  - Buffer for unknowns
  - User feedback
  - Bug fixes
```

---

## Open Questions

1. **Scope: Windows Support?**
   - MVP is Arch Linux only. Windows requires testing UAC, path handling, packaging.
   - Defer to post-MVP unless critical for market.

2. **Skill Editing: Plain Text or Live Preview?**
   - MVP: Plain text (CodeMirror) with syntax highlighting.
   - Post-MVP: Add rendered Markdown preview in split pane.

3. **Auto-Updates: Built-In or Distro Package Manager?**
   - MVP: None (ship AppImage + GitHub Releases).
   - Post-MVP: Integrate `tauri-plugin-updater` for self-updates.

4. **Remote Servers: Include in MVP?**
   - MVP: No (Chops supports OpenClaw/Hermes remote servers, but this is niche).
   - Post-MVP: Add SSH-based skill sync.

5. **Performance: Virtual Scrolling for Large Lists?**
   - MVP: No (Svelte's built-in rendering handles 500+ skills fine).
   - Post-MVP: Add if performance degrades with 1000+ skills.

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| **Rust learning curve** | Medium | High | Hire/pair with Rust expert; start with simple modules |
| **Tauri bugs/limitations** | Low | Medium | Tauri is stable; fallback to Electron if critical issue |
| **Path discovery failures** | Medium | Medium | Comprehensive testing on Arch + Ubuntu; fallback to manual config |
| **File watcher race conditions** | Low | Low | Debounce + file hashing to detect real changes |
| **SQLite concurrency issues** | Low | Low | Use `sqlx` with connection pools; avoid concurrent writes |
| **UI Polish (responsiveness)** | Medium | Low | Use Svelte's reactivity; test on low-end hardware |

---

## Success Looks Like

1. **Shipping:** AppImage + AUR PKGBUILD available on GitHub Releases
2. **Users:** Arch Linux developer installs with `yay -S chops-linux-bin`
3. **Feature Parity:** Can do everything the macOS version can (except remote servers)
4. **Performance:** Snappy, responsive, no lag on 500+ skills
5. **Maintainability:** Codebase is well-organized, easy to extend
6. **Community:** First Linux port of a popular AI agent skill manager

---

## Next Steps (For the Agent)

1. **Environment Setup**
   - Install Rust (rustup), Node (pnpm), Tauri CLI
   - Clone/fork the original Chops repo for reference

2. **Spike: Porting SkillScanner**
   - Pick one Swift file; convert to Rust
   - Test on real Arch Linux system
   - Identify unknowns (path handling, symlinks, etc.)

3. **Tauri Scaffolding**
   - Create Tauri project structure
   - Set up Cargo.toml with dependencies
   - Get "Hello World" command working

4. **Iterative Development**
   - Build backend → expose as Tauri command → test frontend
   - Rinse & repeat for scanner, parser, editor, collections

5. **Testing & Feedback**
   - QA on Arch Linux
   - Gather user feedback
   - Iterate on UX

---

## Appendix: Skill File Format

Chops works with these skill formats:

### Markdown with YAML Frontmatter (.md)
```markdown
---
name: "Extract Variables"
description: "Extract variables from code snippets"
tags: ["refactor", "variables"]
---

# Skill Description

Look for assignments and extract them into variables.
```

### Cursor .mdc Format
```
# Skill Name
description: Extract variables from code

content here
```

### TOML (Codex)
```toml
[metadata]
name = "Extract Variables"
description = "Extract variables from code snippets"

[content]
text = """
Look for assignments...
"""
```

The **SkillParser** detects format by extension and delegates to the appropriate parser. The Linux port must support all three.

---

## Final Thought

This isn't a "nice-to-have" feature. It's **market expansion.** The Chops team deliberately chose macOS-only for speed-to-market. But now that the product is proven, a Linux port isn't just feasible—it's strategic. Linux developers represent a large, underserved segment. Tauri makes this port **lightweight and maintainable**, not a burden.

**Ship it.**

