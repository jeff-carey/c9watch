---
name: release
description: Use when cutting or publishing a new release of this modded c9watch fork — producing distributable macOS DMG/app builds for coworkers, tagging/publishing a "-mods" build, or when the user runs /release.
---

# Cutting a modded release

Publishes a release of this fork to **GitHub Releases** via the tag-triggered
Actions pipeline (`.github/workflows/release.yml`). Binaries are Release
**assets** — never committed to the repo. Releases are built from the
**`modded`** branch (= `personal` + the topic branches).

Repo path: `~/PycharmProjects/personal/c9watch` · fork: `jeff-carey/c9watch`.

## Preconditions — fork-only invariants (verify; never undo)

The pipeline only succeeds because of three fork-only fixes on `personal`.
A release with any of them missing fails in CI. Verify before tagging:

| Invariant | Check (expect) | Fix if broken |
|---|---|---|
| Actions enabled on the fork | `gh workflow list --repo jeff-carey/c9watch` → non-empty | enable once in the fork's **Actions** tab (no API) |
| `build-macos` step has **no** signing env | `git show modded:.github/workflows/release.yml \| grep -A6 'Build Tauri app' \| grep -c APPLE_CERTIFICATE` → `0` | remove the `env:` block from that step on `personal` |
| Updater artifacts disabled | `git show modded:src-tauri/tauri.conf.json \| jq -r .bundle.createUpdaterArtifacts` → `false` | set it to `false` on `personal` |

Fix on `personal`, re-derive `modded` (step 1), then continue.

## Steps

Set `R=~/PycharmProjects/personal/c9watch` first.

1. **Make `modded` current.** If any source changed since the last release,
   rebuild it (`modded` is derived — never hand-edit it):
   ```bash
   git -C "$R" checkout main -q && git -C "$R" branch -f modded main && git -C "$R" checkout modded -q
   git -C "$R" merge --no-edit personal \
     feature/terminal-app-exact-tab-focus fix/agents-json-optional-pid feature/account-indicator
   git -C "$R" push --force-with-lease origin modded
   ```
   The merge list = `personal` + every *unmerged* topic branch. **Drop a topic
   branch once it merges upstream** (it then arrives via `main`).

2. **Pick the next version** — `v<upstream-version>-mods.<n>`:
   ```bash
   BASE=$(git -C "$R" show modded:src-tauri/tauri.conf.json | jq -r .version)   # e.g. 0.8.1
   N=$(( $(gh release list --repo jeff-carey/c9watch --json tagName \
        -q "[.[].tagName | select(startswith(\"v$BASE-mods.\"))] | length") + 1 ))
   TAG="v$BASE-mods.$N"; echo "$TAG"
   ```

3. **Tag `modded` and push** (triggers the Release workflow):
   ```bash
   git -C "$R" tag "$TAG" modded && git -C "$R" push origin "$TAG"
   ```

4. **Watch the run.** It builds both arches + CLI, then creates a **draft**
   release (upstream sets `draft: true` by design — build, then review):
   ```bash
   sleep 8
   RUN=$(gh run list --repo jeff-carey/c9watch --workflow release.yml --limit 1 --json databaseId -q '.[0].databaseId')
   gh run watch "$RUN" --repo jeff-carey/c9watch --interval 20
   gh run view "$RUN" --repo jeff-carey/c9watch --json conclusion -q .conclusion   # want: success
   ```
   On failure: `gh run view "$RUN" --repo jeff-carey/c9watch --log-failed | grep -iE 'error|failed'` → see **Common failures**.

5. **Publish the draft as a pre-release.** Always `--prerelease` — these are
   unofficial and ad-hoc signed:
   ```bash
   gh release edit "$TAG" --repo jeff-carey/c9watch --draft=false --prerelease
   ```

6. **Verify + report the URL:**
   ```bash
   gh release view "$TAG" --repo jeff-carey/c9watch --json isDraft,isPrerelease,url,assets \
     -q '"draft:\(.isDraft) prerelease:\(.isPrerelease) assets:\(.assets|length)", .url'
   ```
   Expect `draft:false prerelease:true assets:8` (2 `.dmg`, 2 `.app.tar.gz`, 3 CLI, `latest.json`).

## Common failures

Each maps to an invariant above; all are already fixed, but if a run fails:

| Run-log symptom | Cause | Fix (on `personal`, then re-derive `modded`) |
|---|---|---|
| `security import: failed to import keychain certificate` | empty `APPLE_*` secrets → tauri tries to codesign | remove the signing `env:` from the build-macos step |
| `A public key has been found, but no private key` | updater artifacts need a signing key | set `bundle.createUpdaterArtifacts: false` in `tauri.conf.json` |
| no workflow run triggers at all | fork Actions not enabled | enable once in the Actions tab (UI only) |

## Notes

- The tag is a frozen snapshot; re-deriving `modded` afterward is fine and does
  not affect a published release.
- Never mark a modded release `latest` / non-prerelease — it's an unofficial fork build.
- Do not commit `.dmg`/`.app` into the repo. Releases hold binaries.
