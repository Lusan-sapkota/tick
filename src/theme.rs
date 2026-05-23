use egui::{Rounding, Style, Visuals};

// Accent color palette for the app.
pub struct Palette {
    pub accent: egui::Color32,
    pub green: egui::Color32,
    pub yellow: egui::Color32,
    pub red: egui::Color32,
}

pub const PALETTE: Palette = Palette {
    accent: egui::Color32::from_rgb(59, 130, 246),
    green: egui::Color32::from_rgb(34, 197, 94),
    yellow: egui::Color32::from_rgb(234, 179, 8),
    red: egui::Color32::from_rgb(239, 68, 68),
};

// Semantic surface colors
pub const SURFACE: egui::Color32 = egui::Color32::from_rgb(26, 29, 39);
pub const SURFACE_HOVER: egui::Color32 = egui::Color32::from_rgb(34, 38, 47);
pub const SURFACE_ACTIVE: egui::Color32 = egui::Color32::from_rgb(30, 40, 80);
pub const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(15, 17, 23);
pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(225, 228, 234);
pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(139, 143, 168);
pub const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(91, 95, 114);

// Apply the custom dark theme visuals to an egui context.
pub fn apply_theme(ctx: &egui::Context) {
    let mut style = Style {
        visuals: Visuals::dark(),
        ..Default::default()
    };

    let v = &mut style.visuals;

    // Background
    v.window_fill = BACKGROUND;
    v.panel_fill = BACKGROUND;
    v.faint_bg_color = BACKGROUND;

    // Override the extreme_bg / code / window colors too
    v.extreme_bg_color = egui::Color32::from_rgb(10, 11, 16);
    v.code_bg_color = SURFACE;
    v.window_shadow = egui::epaint::Shadow {
        offset: [0.0, 4.0].into(),
        blur: 24.0,
        spread: 0.0,
        color: egui::Color32::from_black_alpha(120),
    };
    v.window_rounding = Rounding::same(10.0);

    // Widget styling
    v.widgets.noninteractive.bg_fill = SURFACE;
    v.widgets.noninteractive.fg_stroke.color = TEXT_SECONDARY;
    v.widgets.noninteractive.rounding = Rounding::same(6.0);

    v.widgets.inactive.bg_fill = SURFACE;
    v.widgets.inactive.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.inactive.rounding = Rounding::same(6.0);
    v.widgets.inactive.weak_bg_fill = SURFACE;

    v.widgets.hovered.bg_fill = SURFACE_HOVER;
    v.widgets.hovered.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.hovered.rounding = Rounding::same(6.0);
    v.widgets.hovered.weak_bg_fill = SURFACE_HOVER;

    v.widgets.active.bg_fill = SURFACE_ACTIVE;
    v.widgets.active.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.active.rounding = Rounding::same(6.0);
    v.widgets.active.weak_bg_fill = SURFACE_ACTIVE;

    v.widgets.open.bg_fill = SURFACE;
    v.widgets.open.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.open.rounding = Rounding::same(6.0);
    v.widgets.open.weak_bg_fill = SURFACE;

    // Selection
    v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(59, 130, 246, 60);
    v.selection.stroke.color = PALETTE.accent;

    // Hyperlinks
    v.hyperlink_color = PALETTE.accent;

    // Text cursor
    v.text_cursor = egui::Style::default().visuals.text_cursor;

    // Other
    v.warn_fg_color = PALETTE.yellow;
    v.error_fg_color = PALETTE.red;

    // Spacing
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.indent = 14.0;

    ctx.set_style(style);
}

// A card frame for task / note items.
pub fn card_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(SURFACE)
        .rounding(Rounding::same(6.0))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .outer_margin(egui::Margin::symmetric(0.0, 2.0))
}

// A subtle accent button (colored background).
pub fn accent_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(text)
            .color(egui::Color32::WHITE)
            .size(13.0),
    )
    .fill(PALETTE.accent)
    .rounding(Rounding::same(6.0))
    .min_size(egui::vec2(60.0, 28.0))
}

// Priority color from level (0-3).
pub fn priority_color(level: i32) -> egui::Color32 {
    match level {
        1 => PALETTE.green,
        2 => PALETTE.yellow,
        3 => PALETTE.red,
        _ => TEXT_MUTED,
    }
}

// Priority label text.
pub fn priority_label(level: i32) -> &'static str {
    match level {
        1 => "Low",
        2 => "Medium",
        3 => "High",
        _ => "None",
    }
}
