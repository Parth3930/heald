# Changelog

All notable changes to Heald are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).  
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.3] — 2026-08-15

### Added
- **Auto-generated Memory Manifest** — `index.md` is now automatically maintained as a structured markdown table of active decisions, memories, tags, timestamps, and summaries.
- Rebuilt and synced across `heald remember`, `heald forget`, `heald sync`, `heald init`, and `heald context`.
- Fixed empty `# summary` placeholder bug in `heald context agents`.

---

## [0.1.0-beta.1] — 2026-08-09

### Added
- `heald init` — scaffolds local `.heald/` and global `~/.heald/`, imports existing skills from all agent harnesses, injects critical Heald instructions into global agent configs (Antigravity, Claude Code, Cursor, Hermes, Codex)
- `heald sync` — compiles local `AGENTS.md` from rules + skills. Generates `~/.heald/AGENTS.md` as the canonical universal reference
- `heald context agents [--budget <tokens>]` — prints budget-pruned project memory context. Scores documents by pin status and recency. Always includes `index.md`
- `heald remember --type <type> --title <title> --body <body>` — saves a memory document to `.heald/memory/` as plain OKF Markdown
- `heald finalize --summary <summary>` — appends session summary to `.heald/memory/log.md`
- `heald doctor` — validates all OKF files and reports malformed frontmatter
- **Dynamic routing table** — `heald sync` reads skill `triggers:` frontmatter and builds a live routing table with absolute paths to `~/.heald/skills/`. Works across all projects without local skill copies
- **Skill auto-import** — on `heald init`, skills from Antigravity (`~/.gemini/config/skills/`), Cursor (`.cursor/rules/`), and Hermes are automatically ingested into `~/.heald/skills/`
- **OKF frontmatter tolerance** — skills using `name:` instead of `title:`, or missing `type:`, are accepted gracefully
- **Auto-scaffold on use** — `heald remember`, `heald finalize`, and `heald context` auto-initialize `.heald/` if missing, so agents don't fail on uninitiated repos
- **Explicit argument validation** — `heald finalize` and `heald remember` exit with code 1 and a clear error message if required arguments are missing (prevents agent hangs)

### Architecture
- Single Rust binary, zero runtime dependencies
- Storage: plain Markdown + YAML frontmatter (Open Knowledge Format / OKF)
- Global store: `~/.heald/` (rules, skills — shared across all projects)
- Local store: `<project>/.heald/` (per-project memory, project-specific rules)
- One compiled output file per project: `AGENTS.md` in the project root

### Known Limitations
- No file locking — concurrent writes to shared files (`AGENTS.md`, `log.md`) can race
- Codex native format not yet implemented — uses `.agents/AGENTS.md` fallback
- No `heald forget` command yet

---

*Heald is in beta. APIs and file formats may change before 1.0.*
