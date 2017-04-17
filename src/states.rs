use std::default::Default;

#[derive(Clone)]
pub struct StrokeAnchor {
    pub pos: [f32; 2],

    // Range from 0.0 to 1.0
    pub pressure: f32,
}

impl StrokeAnchor {
    pub fn new(pos: &[f32; 2], pressure: f32) -> Self {
        assert!(pressure >= 0.0 && pressure <= 1.0);

        Self {
            pos: *pos,
            pressure: pressure,
        }
    }
}

#[derive(Clone)]
pub struct OneStroke {
    pub color: [f32; 4],
    pub anchors: Vec<StrokeAnchor>,
}

impl OneStroke {
    pub fn clear(&mut self) {
        self.anchors.clear();
    }

    pub fn add_anchor(&mut self, new_anchor: StrokeAnchor) {
        self.anchors.push(new_anchor);
    }
}

#[derive(Clone, Copy)]
pub enum RenderMode {
    BlackAndWhite,
    Colored,
}

pub struct States {
    pub render_mode: RenderMode,

    pub is_recording_trajectory: bool,
    pub current_recording_cooldown: f32,
    pub max_recording_cooldown: f32,

    pub initial_ink_quantity: f32,
    pub ink_quantity_friction: f32,

    pub max_brush_width: f32,

    pub stroke_line_radius: f32,
    pub stroke_interpolation_accuracy: f32,
    pub stroke_speed_factor: f32,

    pub recording_stroke_anchors: OneStroke,
    pub stroke_records: Vec<OneStroke>,

    pub show_anchors: bool,
    pub show_stroke_lines: bool,

    pub show_brush_preview: bool,
    pub show_ink_quantity_preview: bool,
    pub show_stroke_outline_preview: bool,

    pub need_update_brush_preview: bool,
}

impl Default for States {
    fn default() -> Self {
        States {
            render_mode: RenderMode::BlackAndWhite,

            is_recording_trajectory: false,
            current_recording_cooldown: 0.,
            max_recording_cooldown: 0.033,

            initial_ink_quantity: 100.,
            ink_quantity_friction: 1.,

            max_brush_width: 15.,

            stroke_line_radius: 1.0,
            stroke_interpolation_accuracy: 10.,
            stroke_speed_factor: 2.0,

            recording_stroke_anchors: OneStroke {
                color: [0.0; 4],
                anchors: Vec::new(),
            },
            stroke_records: Vec::new(),

            show_anchors: true,
            show_stroke_lines: true,

            show_brush_preview: true,
            show_ink_quantity_preview: false,
            show_stroke_outline_preview: false,

            need_update_brush_preview: false,
        }
    }
}
