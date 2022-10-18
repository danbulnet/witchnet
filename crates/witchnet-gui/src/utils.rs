use bevy::prelude::Color;

use bevy_egui::egui::Color32;

pub fn color_bevy_to_egui(color: &Color) -> Color32 {
    let [r, g, b, a] = color.as_rgba_f32();
    Color32::from_rgba_unmultiplied(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    )
}