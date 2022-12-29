use bevy::prelude::Color;

use bevy_egui::egui::{ plot::{ MarkerShape, LineStyle }, Color32 };

pub fn color_bevy_to_egui(color: &Color) -> Color32 {
    let [r, g, b, a] = color.as_rgba_f32();
    Color32::from_rgba_unmultiplied(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    )
}

pub fn shape_to_string(shape: &MarkerShape) -> String {
    match shape {
        MarkerShape::Circle => "circle".to_string(),
        MarkerShape::Diamond => "diamond".to_string(),
        MarkerShape::Square => "square".to_string(),
        MarkerShape::Cross => "cross".to_string(),
        MarkerShape::Plus => "plus".to_string(),
        MarkerShape::Up => "up".to_string(),
        MarkerShape::Down => "down".to_string(),
        MarkerShape::Left => "left".to_string(),
        MarkerShape::Right => "right".to_string(),
        MarkerShape::Asterisk => "asterisk".to_string(),
    }
}

#[allow(unused)]
pub fn line_style_to_string(shape: &LineStyle) -> String {
    match shape {
        LineStyle::Solid => "solid".to_string(),
        LineStyle::Dashed { length } => "dashed".to_string(),
        LineStyle::Dotted { spacing } => "dotted".to_string()
    }
}

pub fn shrink_str(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        text.to_string()
    } else {
        format!(
            "{}...",
            &text[..text.char_indices().nth(limit - 3).unwrap().0],
        )
    }
}