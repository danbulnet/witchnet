use std::{
    sync::RwLock,
    f64::consts::PI
};

use once_cell::sync::Lazy;

use bevy_egui::egui::{
    Color32,
    plot::{ Polygon, PlotUi, LineStyle }
};

pub struct RoundedCornersR25 {
    pub radius: f64,
    pub top_left: [[f64; 2]; 25],
    pub bottom_left: [[f64; 2]; 25],
    pub bottom_right: [[f64; 2]; 25],
    pub top_right: [[f64; 2]; 25],
}

pub enum CornerPosition {
    TopLeft,
    BottomLeft,
    BottomRight,
    TopRight
}

impl RoundedCornersR25 {
    fn calculate(radius: f64) -> Self {
        let resolution = 25.0;
        let dr = (PI / 2.0) / resolution;
        
        let top_left = {
            let mut points: [[f64; 2]; 25] = [[0.0; 2]; 25];
            let mut n = 0; while n < 25 {
                points[n] = [
                    radius + ((0.5 * PI + n as f64 * dr).cos() * radius), 
                    0.0 - radius + ((0.5 * PI + n as f64 * dr).sin() * radius)
                ];
                n += 1;
            }
            points
        };

        let bottom_left = {
            let mut points: [[f64; 2]; 25] = [[0.0; 2]; 25];
            for n in 0..25 {
                points[n] = [
                    radius + ((PI + n as f64 * dr).cos() * radius), 
                    radius + ((PI + n as f64 * dr).sin() * radius)
                ];
            }
            points
        };

        let bottom_right = {
            let mut points: [[f64; 2]; 25] = [[0.0; 2]; 25];
            for n in 0..25 {
                points[n] = [
                    0.0 - radius + ((1.5 * PI + n as f64 * dr).cos() * radius), 
                    radius + ((1.5 * PI + n as f64 * dr).sin() * radius)
                ];
            }
            points
        };

        let top_right = {
            let mut points: [[f64; 2]; 25] = [[0.0; 2]; 25];
            for n in 0..25 {
                points[n] = [
                    0.0 - radius + ((n as f64 * dr).cos() * radius), 
                    0.0 - radius + ((n as f64 * dr).sin() * radius)
                ];
            }
            points
        };

        RoundedCornersR25 { radius, top_left, bottom_left, bottom_right, top_right }
    }

    fn rounded_corner(&self, position: (f64, f64), corner: CornerPosition) -> Vec<[f64; 2]> {
        match corner {
            CornerPosition::TopLeft => {
                self.top_left.into_iter().map(|x| [x[0] + position.0, x[1] + position.1]).collect()
            }
            CornerPosition::BottomLeft => {
                self.bottom_left.into_iter().map(|x| [x[0] + position.0, x[1] + position.1]).collect()
            }
            CornerPosition::BottomRight => {
                self.bottom_right.into_iter().map(|x| [x[0] + position.0, x[1] + position.1]).collect()
            }
            CornerPosition::TopRight => {
                self.top_right.into_iter().map(|x| [x[0] + position.0, x[1] + position.1]).collect()
            }
        }
    }
}

static ROUNDED_CORNERS_R25R01: Lazy<RwLock<RoundedCornersR25>> = Lazy::new(|| {
    RwLock::new(RoundedCornersR25::calculate(0.1))
});

pub fn rounded_box_r25r01(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64),
    size: (f64, f64),
    rounded: bool,
    color: Color32
) {
    let radius = 0.1;
    let points: Vec<[f64; 2]> = if rounded {
        let mut points: Vec<[f64; 2]> = Vec::new();
        points.push([origin.0 + radius, origin.1]);
        points.push([origin.0 + size.0 - radius, origin.1]);

        points.extend(&mut ROUNDED_CORNERS_R25R01.read().unwrap().rounded_corner(
            (origin.0 + size.0, origin.1),
            CornerPosition::BottomRight
        ).into_iter());
        
        points.push([origin.0 + size.0, origin.1 + radius]);
        points.push([origin.0 + size.0, origin.1 + size.1 - radius]);

        points.extend(&mut ROUNDED_CORNERS_R25R01.read().unwrap().rounded_corner(
            (origin.0 + size.0, origin.1 + size.1),
            CornerPosition::TopRight
        ).into_iter());

        points.push([origin.0 + size.0 - radius, origin.1 + size.1]);
        points.push([origin.0 + radius, origin.1 + size.1]);

        points.extend(&mut ROUNDED_CORNERS_R25R01.read().unwrap().rounded_corner(
            (origin.0, origin.1 + size.1),
            CornerPosition::TopLeft
        ).into_iter());

        points.push([origin.0, origin.1 + size.1 - radius]);
        points.push([origin.0, origin.1 + radius]);

        points.extend(&mut ROUNDED_CORNERS_R25R01.read().unwrap().rounded_corner(
            (origin.0, origin.1),
            CornerPosition::BottomLeft
        ).into_iter());

        points
    } else {
        vec![
            [origin.0, origin.1],
            [origin.0 + size.0, origin.1],
            [origin.0 + size.0, origin.1 + size.1],
            [origin.0, origin.1 + size.1]
        ]
    };
    let polygon = Polygon::new(points);
    ui.polygon(
        polygon.name(name)
            .color(color)
            .style(LineStyle::Dotted { spacing: 0.5 }).fill_alpha(0.8)
    );
}

pub fn rounded_box(
    ui: &mut PlotUi, 
    name: &str, 
    origin: (f64, f64),
    size: (f64, f64),
    rounded: Option<(f64, u16)>,
    color: Color32
) {
    let points: Vec<[f64; 2]> = if let Some((radius, resolution)) = rounded {
        let dr = (PI / 2.0) / resolution as f64;
        let mut points = Vec::new();
        points.push([origin.0 + radius, origin.1]);
        points.push([origin.0 + size.0 - radius, origin.1]);

        for n in 0..resolution {
            points.push(
                [
                    origin.0 + size.0 - radius + ((1.5 * PI + n as f64 * dr).cos() * radius), 
                    origin.1 + radius + ((1.5 * PI + n as f64 * dr).sin() * radius)
                ]
            );
        }
        
        points.push([origin.0 + size.0, origin.1 + radius]);
        points.push([origin.0 + size.0, origin.1 + size.1 - radius]);

        for n in 0..resolution {
            points.push(
                [
                    origin.0 + size.0 - radius + ((n as f64 * dr).cos() * radius), 
                    origin.1 + size.1 - radius + ((n as f64 * dr).sin() * radius)
                ]
            );
        }

        points.push([origin.0 + size.0 - radius, origin.1 + size.1]);
        points.push([origin.0 + radius, origin.1 + size.1]);

        for n in 0..resolution {
            points.push(
                [
                    origin.0 + radius + ((0.5 * PI + n as f64 * dr).cos() * radius), 
                    origin.1 + size.1 - radius + ((0.5 * PI + n as f64 * dr).sin() * radius)
                ]
            );
        }

        points.push([origin.0, origin.1 + size.1 - radius]);
        points.push([origin.0, origin.1 + radius]);

        for n in 0..resolution {
            points.push(
                [
                    origin.0 + radius + ((PI + n as f64 * dr).cos() * radius), 
                    origin.1 + radius + ((PI + n as f64 * dr).sin() * radius)
                ]
            );
        }

        points
    } else {
        vec![
            [origin.0, origin.1],
            [origin.0 + size.0, origin.1],
            [origin.0 + size.0, origin.1 + size.1],
            [origin.0, origin.1 + size.1]
        ]
    };
    let polygon = Polygon::new(points);
    ui.polygon(
        polygon.name(name)
            .color(color)
            .style(LineStyle::Dotted { spacing: 0.5 }).fill_alpha(0.5)
    );
}