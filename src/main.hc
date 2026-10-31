import "imgui"

fun main() {
  gui_window("Hica Launcher", 900, 600, () => {
    gui_set_color_text(0.92, 0.94, 0.97)
    gui_set_color_bg(0.09, 0.12, 0.17)
    gui_set_color_surface(0.14, 0.18, 0.24)
    gui_set_color_accent(0.24, 0.63, 0.80)
    gui_set_style_window_padding(24.0, 20.0)

    gui_text("Hica Launcher")
    gui_separator()
    gui_spacing()
    gui_text("Applications")
    gui_spacing()
    gui_text_wrapped("No applications in the catalog yet.")
    gui_text_wrapped("Curated Hica applications will appear here when added.")
    gui_spacing()
    gui_text("Downloads and installation are not available yet.")
  })
}
