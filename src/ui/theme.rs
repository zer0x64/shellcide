use chrono::{DateTime, Utc};
use eframe::egui::{self, Color32, Stroke};

pub const CYBER_CYAN: Color32 = Color32::from_rgb(0, 206, 201);
pub const NEON_PINK: Color32 = Color32::from_rgb(253, 121, 168);
pub const SOFT_ORANGE: Color32 = Color32::from_rgb(250, 177, 160);
pub const ICE_BLUE: Color32 = Color32::from_rgb(116, 185, 255);
pub const TOXIC_GREEN: Color32 = Color32::from_rgb(85, 239, 196);
pub const SLATE_GRAY: Color32 = Color32::from_rgb(99, 110, 114);
pub const BRIGHT_RED: Color32 = Color32::from_rgb(255, 118, 117);
pub const MUSTARD_YELLOW: Color32 = Color32::from_rgb(254, 202, 87);
pub const SOFT_GREEN: Color32 = Color32::from_rgb(180, 210, 180);
pub const LAUGHTER_PURPLE: Color32 = Color32::from_rgb(162, 155, 254);
pub const SOFT_WHITE: Color32 = Color32::from_rgb(223, 230, 233);
pub const CHARCOAL: Color32 = Color32::from_rgb(33, 38, 45);
pub const GLOOM_GRAY: Color32 = Color32::from_rgb(48, 54, 61);
pub const LIGHT_GRAY: Color32 = Color32::from_rgb(201, 209, 217);

pub(crate) fn header_label(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).strong().color(CYBER_CYAN));
}

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
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, LIGHT_GRAY);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, GLOOM_GRAY);

    // Button states
    visuals.widgets.inactive.bg_fill = CHARCOAL;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, LIGHT_GRAY);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, GLOOM_GRAY);

    // Hover styles: Highlights with Cyber Cyan Glow
    visuals.widgets.hovered.bg_fill = GLOOM_GRAY;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, CYBER_CYAN);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, CYBER_CYAN);

    // Active styles: Pressed buttons
    visuals.widgets.active.bg_fill = Color32::from_rgb(56, 62, 70);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, CYBER_CYAN);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, CYBER_CYAN);

    style.visuals = visuals;
    ctx.set_style(style);
}

pub fn lerp_color(from: Color32, to: Color32, t: f32) -> Color32 {
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
    Color32::from_rgb(
        lerp(from.r(), to.r()),
        lerp(from.g(), to.g()),
        lerp(from.b(), to.b()),
    )
}

/// Calculates the animation/fade-out factor (1.0 down to 0.0) based on an optional timestamp and duration.
/// Automatically requests a repaint of the UI context if the animation is active.
pub fn get_animation_factor(
    last_changed: Option<DateTime<Utc>>,
    duration_secs: f32,
    ctx: &egui::Context,
) -> f32 {
    if let Some(t) = last_changed {
        let elapsed = (chrono::Utc::now() - t).num_milliseconds() as f32 / 1000.0;
        if elapsed < duration_secs {
            ctx.request_repaint();
            1.0 - (elapsed / duration_secs)
        } else {
            0.0
        }
    } else {
        0.0
    }
}
