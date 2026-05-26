use eframe::egui::{self, Color32, Stroke};

/// Custom Styling settings for Cyber Cyan dark theme interface.
pub fn apply_cyber_cyan_theme(ctx: &egui::Context) {
    use egui::Visuals;

    let mut style = (*ctx.style()).clone();
    let mut visuals = Visuals::dark();

    // Dark grey and solid black background panels
    visuals.window_fill = Color32::from_rgb(12, 16, 21);
    visuals.panel_fill = Color32::from_rgb(12, 16, 21);
    visuals.extreme_bg_color = Color32::from_rgb(22, 27, 34);

    // Non-interactive items
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(22, 27, 34);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(201, 209, 217));
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(48, 54, 61));

    // Button states
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(33, 38, 45);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(201, 209, 217));
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(48, 54, 61));

    // Hover styles: Highlights with Cyber Cyan Glow
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(48, 54, 61);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 206, 201));
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 206, 201));

    // Active styles: Pressed buttons
    visuals.widgets.active.bg_fill = Color32::from_rgb(56, 62, 70);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 206, 201));
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 206, 201));

    style.visuals = visuals;
    ctx.set_style(style);
}
