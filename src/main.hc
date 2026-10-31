import "imgui"

fun apply_default_theme() {
  gui_set_color_text(0.118, 0.161, 0.231)
  gui_set_color_bg(0.902, 0.914, 0.933)
  gui_set_color_surface(0.973, 0.976, 0.980)
  gui_set_color_border(0.886, 0.910, 0.941)
  gui_set_color_accent(0.310, 0.275, 0.898)
  gui_set_color_plot(0.031, 0.569, 0.698)
  gui_set_color_plot_bar(0.918, 0.345, 0.047)
  gui_set_color_modal_dim(0.20)
  gui_set_style_rounding(8.0, 5.0, 5.0)
  gui_set_style_padding(10.0, 5.0)
  gui_set_style_window_padding(14.0, 12.0)
  gui_set_style_spacing(8.0, 6.0, 18.0)
  gui_set_style_borders(1.0, 0.0)
}

fun main() {
  gui_window("Hica Launcher", 900, 600, () => {
    apply_default_theme()

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
