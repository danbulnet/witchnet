use bevy_egui::egui::*;

use super::*;

pub enum NodeShape {
    RoundedRectangle,
    Circle
}

pub struct Nodes {
    pub(crate) series: PlotPoints,
    pub(crate) shape: NodeShape,
    /// Color of the marker. `Color32::TRANSPARENT` means that it will be picked automatically.
    pub(crate) color: Color32,
    pub(crate) filled: bool,
    pub(crate) radius: f32,
    pub(crate) name: String,
    pub(crate) highlight: bool,
}

impl Nodes {
    pub fn new(series: impl Into<PlotPoints>) -> Self {
        Self {
            series: series.into(),
            shape: NodeShape::RoundedRectangle,
            color: Color32::TRANSPARENT,
            filled: true,
            radius: 1.0,
            name: Default::default(),
            highlight: false
        }
    }

    /// Set the shape of the markers.
    pub fn shape(mut self, shape: NodeShape) -> Self {
        self.shape = shape;
        self
    }

    /// Highlight these points in the plot by scaling up their markers.
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Set the marker's color.
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    /// Whether to fill the marker.
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }


    /// Set the maximum extent of the marker around its position.
    pub fn radius(mut self, radius: impl Into<f32>) -> Self {
        self.radius = radius.into();
        self
    }

    /// Name of this set of points.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Multiple plot items may share the same name, in which case they will also share an entry in
    /// the legend.
    #[allow(clippy::needless_pass_by_value)]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
}

impl PlotItem for Nodes {
    fn shapes(&self, _ui: &mut Ui, transform: &ScreenTransform, shapes: &mut Vec<Shape>) {
        let Self {
            series,
            shape,
            color,
            filled,
            mut radius,
            highlight,
            ..
        } = self;

        let stroke_size = radius / 5.0;

        let default_stroke = Stroke::new(stroke_size, *color);
        let mut stem_stroke = default_stroke;
        let stroke = (!filled)
            .then(|| default_stroke)
            .unwrap_or_else(Stroke::none);
        let fill = filled.then(|| *color).unwrap_or_default();

        if *highlight {
            radius *= 2f32.sqrt();
            stem_stroke.width *= 2.0;
        }

        series
            .points()
            .iter()
            .map(|value| {
                let value_min = PlotPoint { x: value.x, y: value.y };
                let value_max = PlotPoint { 
                    x: value.x + radius as f64, y: value.y - radius as f64 
                };
                let value_min_transofrmed = transform.position_from_point(&value_min);
                let value_max_transofrmed = transform.position_from_point(&value_max);
                (value_min_transofrmed, value_max_transofrmed)
            })
            .for_each(|(min, max)| {
                match shape {
                    NodeShape::RoundedRectangle => {
                        shapes.push(Shape::Rect(epaint::RectShape {
                            rect: Rect { min, max },
                            rounding: Rounding { nw: 1.5, ne: 1.5, sw: 1.5, se: 1.5 },
                            fill,
                            stroke,
                        }));
                    }
                    NodeShape::Circle => {
                        shapes.push(Shape::Circle(epaint::CircleShape {
                            center: min,
                            radius: f32::abs(max.x - min.x),
                            fill,
                            stroke,
                        }));
                    }
                }
            });
    }

    fn initialize(&mut self, x_range: RangeInclusive<f64>) {
        self.series.generate_points(x_range);
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn color(&self) -> Color32 {
        self.color
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Points(self.series.points())
    }

    fn bounds(&self) -> PlotBounds {
        self.series.bounds()
    }
}