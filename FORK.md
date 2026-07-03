# This is a modified fork of c9watch

A personal build of [`minchenlee/c9watch`](https://github.com/minchenlee/c9watch)
with a few local enhancements. **The `modded` branch (the default) is the
complete, buildable modified app.**

## What's modified

- **Exact Terminal.app tab focus** — clicking a session raises the *specific*
  Terminal.app window/tab hosting it, instead of only bringing the app forward.
  (Submitted upstream as [PR #109](https://github.com/minchenlee/c9watch/pull/109).)
- **Account indicator** — the status bar shows which Claude account your sessions
  are logged into (helpful when juggling work and personal accounts).
- **Detector fix** — the session detector no longer crashes when
  `claude agents --json` lists a background agent without a `pid`.

## Grab a prebuilt build (easiest)

Prebuilt `.dmg` / `.app` bundles for both architectures are attached to the
fork's releases:

**→ https://github.com/jeff-carey/c9watch/releases**

Download the `.dmg` matching your Mac (`aarch64` for Apple Silicon, `x86_64`
for Intel — the x86_64 build also runs on Apple Silicon via Rosetta 2). These
are **ad-hoc signed, not notarized**, so clear the quarantine flag once after
installing:

```bash
xattr -dr com.apple.quarantine /Applications/c9watch.app
```

(or right-click the app → **Open** → **Open**). On first launch, approve the
macOS "control Terminal.app" prompt so click-to-focus works.

## Or build it yourself

Building locally gives you a native binary and avoids the quarantine step.

**Prerequisites:** [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/)
v18+, and Xcode Command Line Tools (`xcode-select --install`).

```bash
git clone https://github.com/jeff-carey/c9watch.git
cd c9watch
git checkout modded          # the default branch; the full modded app
npm install
npm run tauri build
# then move the built app into place:
#   src-tauri/target/release/bundle/macos/c9watch.app  ->  /Applications
```

The build produces a binary matching *your* Mac's architecture. On first
launch, approve the macOS "control Terminal.app" prompt so click-to-focus works.

## Notes

- Upstream: [minchenlee/c9watch](https://github.com/minchenlee/c9watch). As the
  changes above merge upstream, they'll disappear from this fork and you can go
  back to official notarized releases.
- Development/branch layout is documented in [CLAUDE.md](CLAUDE.md).
