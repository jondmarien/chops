# Chops Linux Port: Agent Handoff

## Mission
Port the macOS-native Chops application to Linux (Arch Linux first, then other major distros) using **Tauri** as the runtime. Target: ~7 MB binary footprint, ~60% code reuse from the original codebase, MVP delivery in 6–8 weeks.

## High-Level Architecture

### Tech Stack
- **Frontend:** Tauri (Rust) + Svelte + TypeScript
- **Backend:** Rust (business logic extracted from Swift)
- **Database:** SQLite (via `sqlx` or `rusqlite`)
- **File Watching:** `notify-rs` crate (cross-platform FSEvents/inotify/ReadDirectoryChangesW)
- **Text Editor:** CodeMirror or Monaco (syntax highlighting, skill editing)
- **Search:** Fuse.js (full-text search on the frontend)

### What Gets Ported (60% reuse)
1. **SkillScanner** (`Chops/Services/SkillScanner.swift`) — filesystem probing, skill discovery
2. **SkillParser** + **FrontmatterParser** (`Chops/Utilities/`) — YAML/Markdown parsing
3. **ToolSource enum** (`Chops/Models/ToolSource.swift`) — tool detection, path management
4. **AppState** — UI state management logic
5. **Models** (`Skill`, `Collection`, `ToolSource`) — domain objects

### What Gets Rebuilt (40% new code)
1. **UI Layer** (SwiftUI → Svelte) — three-column layout, editor, forms
2. **Persistence** (SwiftData → SQLite) — schema, migrations, queries
3. **File Watching** (FSEvents → `notify-rs`) — real-time sync
4. **IPC/Tauri Commands** — Rust-to-frontend communication layer
5. **Packaging & Distribution** — AppImage, PKGBUILD, GitHub Releases

---

## Repository Structure (Post-Port)

```
chops-linux/
├── src-tauri/                          # Rust backend
│   ├── src/
│   │   ├── main.rs                     # Tauri entry point, IPC handlers
│   │   ├── scanner.rs                  # Ported SkillScanner
│   │   ├── parser.rs                   # Ported SkillParser + FrontmatterParser
│   │   ├── db.rs                       # SQLite schema, queries
│   │   ├── db/
│   │   │   ├── models.rs               # Skill, Collection, ToolSource structs
│   │   │   ├── migrations.rs           # Schema version management
│   │   │   └── queries.rs              # CRUD operations
│   │   ├── watcher.rs                  # File watching (notify-rs)
│   │   ├── tools.rs                    # Ported ToolSource enum
│   │   ├── errors.rs                   # Error types
│   │   └── lib.rs                      # Public API
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                                # Svelte/TypeScript frontend
│   ├── App.svelte                      # Root component
│   ├── routes/
│   │   ├── +page.svelte                # Main three-column layout
│   │   └── settings/+page.svelte       # Settings view
│   ├── components/
│   │   ├── Sidebar.svelte              # Tool filters, collections
│   │   ├── SkillList.svelte            # Skill list with search
│   │   ├── SkillEditor.svelte          # Markdown editor (CodeMirror)
│   │   ├── SkillDetail.svelte          # Skill metadata display
│   │   ├── NewSkillDialog.svelte       # Skill creation form
│   │   └── ToolBadge.svelte            # Tool icon/label component
│   ├── stores/
│   │   ├── app.ts                      # AppState (filters, selection, search)
│   │   ├── skills.ts                   # Skill list state
│   │   ├── collections.ts              # Collections state
│   │   └── ui.ts                       # UI flags (loading, errors)
│   ├── lib/
│   │   ├── tauri.ts                    # IPC command wrappers
│   │   └── utils.ts                    # Formatting, helpers
│   └── styles/
│       └── main.css                    # Tailwind/global styles
│
├── Cargo.toml                          # Workspace manifest
├── package.json                        # Node dependencies
├── pnpm-lock.yaml                      # Lock file
├── tailwind.config.js                  # Tailwind configuration
├── svelte.config.js                    # Svelte configuration
└── vite.config.ts                      # Vite bundler config
```

---

## Tauri Commands (IPC Interface)

The backend exposes these commands for the frontend to call:

### Scanner
- `scan_all()` → `[Skill]` — Full filesystem scan
- `get_skills(filters: SkillFilters)` → `[Skill]` — Get skills with filtering
- `get_skill(id: String)` → `Skill` — Fetch single skill

### Editor
- `save_skill(id: String, content: String, frontmatter: Map<String, String>)` → `Skill` — Write skill to disk
- `create_skill(tool: String, name: String, content: String)` → `Skill` — Create new skill
- `delete_skill(id: String)` → `void` — Delete skill from disk
- `get_skill_content(id: String)` → `String` — Raw file content

### Collections
- `create_collection(name: String)` → `Collection` — New collection
- `add_skill_to_collection(skill_id: String, collection_id: String)` → `void`
- `remove_skill_from_collection(skill_id: String, collection_id: String)` → `void`
- `get_collections()` → `[Collection]`

### Settings
- `add_custom_scan_path(path: String)` → `void`
- `remove_custom_scan_path(path: String)` → `void`
- `get_settings()` → `AppSettings`
- `set_setting(key: String, value: Any)` → `void`

### File Watching
- `start_watch()` → `void` — Begin monitoring configured paths
- `stop_watch()` → `void` — Stop file watcher
- `rescan_on_change()` — (Invoked by watcher, triggers `scan_all()`)

---

## Key Design Decisions

### 1. Database Schema (SQLite)
Mirror the SwiftData models:

```sql
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    resolved_path TEXT UNIQUE NOT NULL,
    tool_source TEXT NOT NULL,
    is_directory BOOLEAN NOT NULL,
    is_global BOOLEAN NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    content TEXT,
    frontmatter JSON,
    installed_paths JSON,      -- Array of paths where this skill is symlinked
    tool_sources JSON,         -- Array of ToolSource enums
    file_modified_date DATETIME,
    file_size INTEGER,
    item_kind TEXT,            -- "skill" | "agent" | "rule"
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE collections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE skill_collections (
    skill_id TEXT NOT NULL REFERENCES skills(id),
    collection_id TEXT NOT NULL REFERENCES collections(id),
    PRIMARY KEY (skill_id, collection_id)
);

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 2. Linux Path Conventions (XDG Base Directory Spec)

```rust
// Respect XDG_CONFIG_HOME, XDG_DATA_HOME, XDG_CACHE_HOME
// Fallback to ~/.config, ~/.local/share, ~/.cache if unset

let config_home = env::var("XDG_CONFIG_HOME")
    .unwrap_or_else(|_| format!("{}/.config", home));

let data_home = env::var("XDG_DATA_HOME")
    .unwrap_or_else(|_| format!("{}/.local/share", home));

// Example:
// Claude skills: ~/.config/claude/skills OR ~/.claude/skills (backward-compat)
// Chops data: ~/.local/share/chops/skills.db
```

### 3. Tool Detection Strategy

Each tool's `isInstalled` check uses a cascading approach:

1. Check for app bundle / executable in `$PATH`
2. Check for tool config files (e.g., `~/.claude/settings.json`)
3. Check for presence of skills/agents directories
4. If any check passes, tool is "installed"

This handles cross-platform variability (macOS vs. Linux binary locations).

### 4. File Watcher (notify-rs)

Use `notify::RecommendedWatcher` for automatic platform-specific implementation:

```rust
use notify::{Watcher, RecommendedWatcher, RecursiveMode};

fn start_watcher(paths: Vec<String>, tx: Sender<String>) -> RecommendedWatcher {
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            match res {
                Ok(event) => {
                    // Debounce & trigger rescan
                    tx.send(event.to_string()).ok();
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        },
        notify::Config::default().with_poll_interval(Duration::from_millis(500)),
    ).unwrap();
    
    for path in paths {
        watcher.watch(Path::new(&path), RecursiveMode::Recursive).ok();
    }
    
    watcher
}
```

### 5. Frontend State Management (Svelte Stores)

```typescript
// src/stores/app.ts
export const appState = writable({
  selectedSkill: null,
  selectedCollection: null,
  searchText: '',
  toolFilters: [],    // Which tools to show
  sortBy: 'name',
  isDarkMode: false,
});

export const skills = writable<Skill[]>([]);
export const collections = writable<Collection[]>([]);
export const isLoading = writable(false);
export const error = writable<string | null>(null);
```

### 6. Skill Editor Integration

Use **CodeMirror 6** or **Monaco Editor**:
- Syntax highlighting for Markdown + YAML frontmatter
- Cmd+S (or Ctrl+S on Linux) to save
- Real-time line count and preview
- Diff highlighting if file changed on disk

---

## Supported Tools (Initial Release)

Scan these directories, auto-detect installation:

| Tool | Skills Path | Agents Path | Rules Path |
|------|-------------|------------|-----------|
| **Claude Code** | `~/.claude/skills/` | `~/.claude/agents/` | — |
| **Cursor** | `~/.cursor/skills/` | `~/.cursor/agents/` | `~/.cursor/rules/` |
| **Windsurf** | — | — | `~/.windsurf/rules/` |
| **Codex** | `~/.codex/skills/` | `~/.codex/agents/` | — |
| **Amp** | `$XDG_CONFIG_HOME/amp/skills/` | — | — |
| **Global** | `~/.agents/skills/` | — | — |

(Copilot, Aider, Hermes, OpenClaw support can be added post-MVP.)

---

## Effort Estimate & Timeline

| Phase | Task | Effort | Timeline |
|-------|------|--------|----------|
| **Setup** | Tauri scaffolding, Cargo/npm config | 2–3 days | Week 1 |
| **Backend** | Port SkillScanner, parser, ToolSource to Rust | 4–5 days | Week 1–2 |
| **Database** | SQLite schema, migrations, query layer | 2–3 days | Week 2 |
| **File Watcher** | `notify-rs` integration, debouncing | 2–3 days | Week 2 |
| **IPC Layer** | Tauri commands, state synchronization | 3–4 days | Week 2–3 |
| **UI (Svelte)** | Layout, three-column split, navigation | 3–4 days | Week 3 |
| **Skill List** | List rendering, filtering, search | 2–3 days | Week 3–4 |
| **Skill Editor** | CodeMirror, save flow, validation | 3–4 days | Week 4 |
| **Collections** | Create/edit/delete, skill association | 2–3 days | Week 4–5 |
| **Settings & UX** | Preferences, error handling, edge cases | 2–3 days | Week 5 |
| **Testing** | Arch + Ubuntu, bug fixes, polish | 3–5 days | Week 5–6 |
| **Packaging** | AppImage, AUR PKGBUILD, CI/CD | 2–3 days | Week 6–7 |
| **Documentation** | README, build instructions, screenshots | 1–2 days | Week 7 |
| **Contingency** | Buffer for unknowns | 1–2 weeks | Throughout |
| **Total** | | **6–8 weeks** | |

---

## Critical Dependencies

### Rust Crates
```toml
[dependencies]
tauri = "2.0"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "sqlite"] }
notify-rs = "5"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
regex = "1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
thiserror = "1"
walkdir = "2"
which = "5"
```

### Node Dependencies
```json
{
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^latest",
    "tauri-cli": "^latest",
    "typescript": "^latest",
    "vite": "^latest",
    "svelte": "^latest"
  },
  "dependencies": {
    "codemirror": "^6",
    "fuse.js": "^latest",
    "tailwindcss": "^latest",
    "uuid": "^latest"
  }
}
```

---

## Known Challenges & Mitigation

### 1. **Platform Path Variability**
   - **Issue:** Tool paths differ between macOS (`/Applications/`) and Linux (`~/.local/bin/`, `$PATH`)
   - **Mitigation:** Implement cascading path checker using `which` crate, XDG Base Directory spec

### 2. **File Watching Edge Cases**
   - **Issue:** `notify-rs` on Linux may batch or delay events; editors may write atomically
   - **Mitigation:** Debounce with 500ms; use file hashing to detect actual changes

### 3. **Symlink Handling**
   - **Issue:** Skills can be symlinked across tools; need deduplication
   - **Mitigation:** Use `resolved_path` as canonical identity (resolved symlinks), store multiple `installed_paths`

### 4. **Database Migrations**
   - **Issue:** Schema evolution as features are added
   - **Mitigation:** Use `sqlx` compile-time checked migrations (`.sql` files in `migrations/`)

### 5. **UI Responsiveness with Large Skill Collections**
   - **Issue:** 500+ skills = slow list rendering if naive
   - **Mitigation:** Virtual scrolling (Svelte component), lazy-load content, memoize search results

### 6. **Packaging for Multiple Distros**
   - **Issue:** Ubuntu, Fedora, Arch all have different package formats
   - **Mitigation:** Ship AppImage as primary (works everywhere); provide AUR PKGBUILD for Arch; offer static tar.gz

---

## Definition of Done (MVP)

### Must Have
- ✅ Scan all configured tool directories on startup
- ✅ Display skills/agents/rules in three-column layout
- ✅ Full-text search (name, description, content)
- ✅ Edit and save skills to disk (Cmd+S / Ctrl+S)
- ✅ Create new skills with tool-specific boilerplate
- ✅ Real-time file watching (rescan on disk changes)
- ✅ Collections (organize skills without modifying source)
- ✅ Settings (custom scan paths, preferences)
- ✅ Runs on Arch Linux (x86_64)
- ✅ ~7 MB binary footprint

### Should Have
- Settings persistence
- Error handling & user-friendly messages
- Keyboard shortcuts (Cmd+S, Cmd+F, etc.)
- Dark mode toggle
- README with build/run instructions

### Could Have (Post-MVP)
- Support for Windows
- Support for other Linux distros (Ubuntu, Fedora, etc.)
- Remote server support (SSH-based skill sync)
- Plugin marketplace integration
- Skill sharing/versioning
- Diff viewer for skill changes

---

## Getting Started (For the Agent)

1. **Create a new Tauri project:**
   ```bash
   npm create tauri-app@latest -- --manager pnpm --ui svelte --typescript
   cd chops-linux
   ```

2. **Set up Rust backend structure:**
   ```bash
   mkdir -p src-tauri/src/{db,commands}
   ```

3. **Port core logic:**
   - Start with `scanner.rs` (extracts logic from `SkillScanner.swift`)
   - Follow with `parser.rs` (from `SkillParser.swift` + `FrontmatterParser.swift`)
   - Port `ToolSource` enum to Rust

4. **Build Tauri commands:**
   - Expose scanner, editor, collections via IPC

5. **Build Svelte UI:**
   - Three-column layout with flex/grid CSS
   - Integrate CodeMirror for editor
   - Wire up Tauri commands to UI actions

6. **Iterate & test:**
   - Build with `tauri dev` (live reload)
   - Test on actual Arch system
   - Gather feedback, refine

---

## References & Resources

- **Tauri Docs:** https://tauri.app/
- **Svelte Docs:** https://svelte.dev/
- **CodeMirror 6:** https://codemirror.net/
- **notify-rs:** https://github.com/notify-rs/notify
- **SQLx:** https://github.com/launchbadge/sqlx
- **XDG Base Directory Spec:** https://specifications.freedesktop.org/basedir-spec/latest/
- **Original Chops Repo:** https://github.com/Shpigford/chops

---

## Questions & Escalations

- **Path discovery on Linux:** How to handle tools installed via snap, flatpak, or distro packages?
- **File watcher performance:** Acceptable debounce interval? Rescan entire directory or just changed file?
- **Skill editing:** Should we support live preview (rendered Markdown) or plain text only?
- **Auto-update:** Ship with integrated update checker or rely on distro package managers?

