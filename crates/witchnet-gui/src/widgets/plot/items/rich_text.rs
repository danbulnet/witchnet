use bevy_egui::egui::*;

use super::*;

pub struct RichText {
    pub text: WidgetText,
    pub position: PlotPoint,
    pub name: String,
    pub highlight: bool,
    pub color: Color32,
    pub anchor: Align2,
    pub text_style: TextStyle,
    pub available_width: f32
}

impl RichText {
    pub fn new(position: PlotPoint, text: impl Into<WidgetText>, ) -> Self {
        Self {
            text: text.into(),
            position,
            name: Default::default(),
            highlight: false,
            color: Color32::TRANSPARENT,
            anchor: Align2::CENTER_CENTER,
            text_style: TextStyle::Small,
            available_width: f32::INFINITY
        }
    }

    /// Highlight this text in the plot by drawing a rectangle around it.
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Text color.
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    /// Anchor position of the text. Default is `Align2::CENTER_CENTER`.
    pub fn anchor(mut self, anchor: Align2) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        self
    }

    pub fn available_width(mut self, available_width: f32) -> Self {
        self.available_width = available_width;
        self
    }

    /// Name of this text.
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

impl PlotItem for RichText {
    fn shapes(&self, ui: &mut Ui, transform: &ScreenTransform, shapes: &mut Vec<Shape>) {
        let color = if self.color == Color32::TRANSPARENT {
            ui.style().visuals.text_color()
        } else {
            self.color
        };

        let galley =
            self.text
                .clone()
                .into_galley(ui, Some(true), self.available_width, self.text_style.clone());

        let pos = transform.position_from_point(&self.position);
        let rect = self
            .anchor
            .anchor_rect(Rect::from_min_size(pos, galley.size()));

        let mut text_shape = epaint::TextShape::new(rect.min, galley.galley);
        if !galley.galley_has_color {
            text_shape.override_text_color = Some(color);
        }
        shapes.push(text_shape.into());

        if self.highlight {
            shapes.push(Shape::rect_stroke(
                rect.expand(2.0),
                1.0,
                Stroke::new(0.5, color),
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

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
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.extend_with(&self.position);
        bounds
    }
}