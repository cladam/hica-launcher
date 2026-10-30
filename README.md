# Hica Launcher

A native Rust/egui desktop hub for applications in the hica ecosystem. 

The first slice provides a grid and sortable table of hica applications and
checks the latest GitHub release for each entry. **Installation, downloads,
updates, launching, and tray operation are not implemented yet.** The app
links to release pages but does not execute downloaded software.

## Add applications

Edit `catalogue.json` and rebuild. The catalogue is embedded in the binary:

```json
{
  "apps": [
    {
      "name": "Your App",
      "repo": "your-github-owner/your-app",
      "description": "A short description"
    }
  ]
}
```

Only compiled user-facing applications with GitHub releases belong in this
catalog; hica libraries are managed separately. The repository starts with an
empty catalog until real hica application repositories are chosen. GitHub
release errors are displayed per app instead of being treated as versions.

## Development

Requires Rust 1.95 or newer.

```sh
cargo run
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

The PRD and development plan under `docs/` are excluded from version control
by the local `.gitignore`.
