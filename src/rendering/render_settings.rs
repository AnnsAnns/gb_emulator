use macroquad::prelude::*;

pub struct GbSettings {
    pub scaling: f32,
    pub palette: [Color; 4],
}

impl Default for GbSettings {
    fn default() -> Self {
        GbSettings {
            scaling: 4.0,
            palette: [
                Color::new(1.00, 1.00, 1.00, 1.00),
                Color::new(0.18, 0.83, 0.18, 1.00),
                Color::new(0.12, 0.54, 0.12, 1.00),
                Color::new(0.06, 0.15, 0.06, 1.00)
            ]
        }
    }
}