# CLAUDE.md — working in this fork

This is **Jeff Carey's personal fork** of [`minchenlee/c9watch`](https://github.com/minchenlee/c9watch),
a macOS menu-bar app (Tauri 2 — Rust backend + SvelteKit frontend) that
monitors and controls all your running Claude Code sessions. This file is
guidance for a human or a Claude Code session working in this repository.

> This file lives only on the `personal` branch (and therefore on `modded`).
> It is intentionally **not** on `main` or any `feature/*` / `fix/*` branch, so
> it never reaches an upstream pull request.

## Branch architecture (read this before committing anything)

| Branch | Role | You commit here? |
|--------|------|------------------|
| `main` | **Pristine mirror of upstream `main`.** Never diverges. | No — only `git fetch upstream` / fast-forward |
| `feature/*`, `fix/*` | **One upstream-bound change each**, cut from `main`. These become PRs. | Yes |
| `personal` | **Fork-only files** that must never go upstream (this file, `FORK.md`, README fork-banner, etc.), cut from `main`. | Yes |
| `modded` | **Derived build branch** = `personal` + every `feature/*`/`fix/*` merged together. The full modded app. **Default branch.** | No — it's rebuilt, not authored |

### Golden rules

1. **Cut every new `feature/*` / `fix/*` branch from `main`**, never from
   `personal` or `modded` — otherwise fork-only files leak into the PR.
2. **Never hand-edit `modded`.** It is derived and disposable. To change what's
   in it, change a *source* (`personal` or a topic branch), then re-derive.
3. **Keep `main` pristine.** No commits of your own; only sync from upstream.
4. **PRs always target `upstream/main`**, and thanks to (1) they only ever
   contain the one topic branch's diff. A fork's default branch does not affect
   PR targeting.

### Re-deriving `modded`

`modded` holds no unique work, so rebuild it from scratch whenever a source
changes:

```bash
git checkout main
git branch -f modded main            # reset the pointer to pristine main
git checkout modded
git merge --no-edit personal feature/terminal-app-exact-tab-focus \
  fix/agents-json-optional-pid feature/account-indicator
git push --force-with-lease origin modded
```

Octopus merge only works while the branches are conflict-free (they currently
touch disjoint files). If two ever conflict, drop to sequential `git merge`
and enable `git config --global rerere.enabled true` so a resolution is
recorded once and replayed on future rebuilds. As topic branches merge
upstream, remove them from the merge list and let them arrive via
`main` instead.

## Commit identity

This repo has a **repo-local** `user.email` of
`12699906+jeff-carey@users.noreply.github.com` (GitHub private noreply).
Contribute as your **personal** self, not Recast. The global git identity
(personal gmail) and the Recast-work identity are handled elsewhere; don't
override this repo's config.

## Building

See **[FORK.md](FORK.md)** for the full build/install walkthrough. Key facts:

- The default toolchain here is **Intel Homebrew Rust/Node under Rosetta**, so a
  plain `npm run tauri build` produces an **x86_64** app (runs on Intel and, via
  Rosetta, Apple Silicon).
- A **native arm64** build uses the separately-installed `rustup` toolchain
  (`~/.cargo/bin`, `--no-modify-path`, does not change the default `cargo`):
  ```bash
  PATH="$HOME/.cargo/bin:$PATH" npm run tauri build -- --target aarch64-apple-darwin
  ```
- The fork sets `bundle.createUpdaterArtifacts: false` in `tauri.conf.json`, so
  builds need no signing key and the app won't auto-update to upstream. (Upstream
  ships `"v1Compatible"`, which errors without `TAURI_SIGNING_PRIVATE_KEY`.)
- **Moving the repo invalidates the Tauri build cache** (it bakes absolute paths
  into `target/`). If a build suddenly can't find a generated permissions file,
  clear `target/*/build/tauri-*` and `target/*/.fingerprint/c9watch-*` (or
  `cargo clean`).

## Testing

- `cargo test` (default features) shows **two failures on this machine** —
  `session::status::tests::test_get_pending_tool_name_mixed_completed_and_pending`
  and `test_unknown_entries_after_tool_use_dont_override_status`. These are
  **environment-dependent, not bugs**: `status.rs` initializes a global
  `PermissionChecker` from `~/.claude/settings.json`, so Jeff's broad allow-list
  makes a "dangerous" test command read as auto-approved. With an empty `$HOME`
  the module passes 32/0, so **CI (a clean runner) is green**.
  To reproduce a clean run: `HOME=$(mktemp -d) CARGO_HOME="$HOME/.cargo" cargo test …`
- CI gates (`.github/workflows/ci.yml`): `svelte-check`, `cargo check`,
  `cargo test`, `npm run build`, and a **CLI-only** `cargo check`/`cargo test`
  (`--no-default-features --features cli`). CI does **not** run `cargo fmt` or
  `cargo clippy` — those are checklist courtesy only, and upstream `main` is
  itself fmt/clippy-dirty, so don't reformat unrelated code.

## Distribution

Local builds are **ad-hoc signed** (not notarized). On another Mac, Gatekeeper
blocks a copied `.app` until quarantine is cleared
(`xattr -dr com.apple.quarantine /Applications/c9watch.app`), or the coworker
builds from source (cleanest — see FORK.md). Every rebuild changes the code
signature, so macOS **re-prompts for "control Terminal.app" Automation consent**
on the next session-focus after a reinstall.

## Cutting a release

Prebuilt binaries are published as **GitHub Releases** on the fork (never
committed to the repo — that would bloat it). The release workflow
(`.github/workflows/release.yml`) runs on a `v*` tag push, builds both
architectures + CLI on GitHub runners, and creates a **draft** release
(upstream sets `draft: true` by design — build then review):

```bash
git tag v0.8.1-mods.2 modded          # convention: v<upstream-version>-mods.<n>
git push origin v0.8.1-mods.2
# ...wait for Actions, then publish the draft as a pre-release:
gh release edit v0.8.1-mods.2 --repo jeff-carey/c9watch --draft=false --prerelease
```

Always `--prerelease` (these are unofficial, ad-hoc signed). The tag naming
(`v<upstream-version>-mods.<n>`) makes the upstream base obvious.

Fork-only prerequisites that make this work — all already applied on `personal`
(don't undo them): Actions must be enabled once in the fork's Actions tab (no
API); the `build-macos` "Build Tauri app" step must NOT pass the empty
`APPLE_*`/`TAURI_SIGNING_*` signing env (tauri would try to codesign and fail);
and `createUpdaterArtifacts` must be `false`. If cutting a release from a Claude
session, prefer the `/release` skill if present.

## Upstream contribution conventions

- **Conventional Commits** for messages and PR titles.
- Upstream **squash-merges**, so the **PR title becomes the commit message** —
  make it a clean `feat:` / `fix:` line.
- Branch naming: `feature/…`, `fix/…`, `docs/…`, `chore/…`.
- PR template lives in `.github/pull_request_template.md`.

## Current fork state

Three modifications, each on its own branch and mirrored into `modded`:

- `feature/terminal-app-exact-tab-focus` — exact Terminal.app tab focus via tty
  matching. **Open PR: [minchenlee/c9watch#109](https://github.com/minchenlee/c9watch/pull/109).**
- `fix/agents-json-optional-pid` — tolerate pid-less background agents in
  `claude agents --json` (the detector crashed on them). *Not yet submitted.*
- `feature/account-indicator` — show the logged-in Claude account in the status
  bar. *Not yet submitted.*
