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

## Build it yourself (recommended)

Building locally gives you a native binary and avoids macOS Gatekeeper prompts.

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

> The build finishes with `A public key has been found, but no private key` —
> that's the harmless updater-signing step; your `.app` is already built.

## Or install a prebuilt `.app`

If Jeff hands you a prebuilt `.app`/`.dmg` instead, it is **ad-hoc signed**
(not notarized), so macOS Gatekeeper will block it until you clear the
quarantine flag once:

```bash
xattr -dr com.apple.quarantine /Applications/c9watch.app
```

(or right-click the app → **Open** → **Open**). Match the architecture to your
Mac — an x86_64 build runs on Apple Silicon via Rosetta 2; an arm64 build only
runs on Apple Silicon.

## Notes

- Upstream: [minchenlee/c9watch](https://github.com/minchenlee/c9watch). As the
  changes above merge upstream, they'll disappear from this fork and you can go
  back to official notarized releases.
- Development/branch layout is documented in [CLAUDE.md](CLAUDE.md).
