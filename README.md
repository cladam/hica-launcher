# hica-launcher

Hica Launcher is a native Hica/ImGui application. It currently opens a
desktop window with an empty catalog state; application discovery, downloads,
and installation are not implemented yet. See `docs/PRD-hica-launcher.md`
if you have the local product requirements document.

## Usage

Requires Hica, SDL2, and the ImGui library configured in `hica.hml`.
The checked-in `lib/libimgui_hica.a` and linker flags currently target macOS
with Homebrew SDL2.

```sh
hica check  # type-check without building
hica build  # compile to binary
hica run    # compile and open the window
```

## Catalog

`catalogue.hml` is the curated application catalog. It is intentionally
empty until application entries are confirmed. The pure parser in
`src/catalogue.hc` is tested, but build-time embedding and UI integration
are not implemented yet; the window still displays the static empty state.

Entries use repeated `@app` elements with three required string fields:

```hml
@app {
	name: "Confirmed app name"
	repo: "owner/repository"
	description: "Application description"
}
```

This is a schema example, not a confirmed application. Properties are
newline-delimited. Validation rejects missing, blank, non-string,
duplicate, and unknown fields, malformed GitHub repository IDs, unexpected
document structures, and duplicate repositories (case-insensitive).
An invalid entry rejects the entire catalog with an error.

```sh
hica test tests/test_catalogue.hc
```
