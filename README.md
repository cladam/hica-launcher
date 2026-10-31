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
