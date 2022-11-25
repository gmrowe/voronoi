pub mod consts {
    use super::Color;

    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    pub const RED: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };

    pub const GREEN: Color = Color {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    };

    pub const BLUE: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };

    pub const MAGENTA: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 1.0,
    };

    pub const CYAN: Color = Color {
        red: 0.0,
        green: 1.0,
        blue: 1.0,
    };

    pub const YELLOW: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 0.0,
    };

    pub const NAVY: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.5,
    };

    pub const TEAL: Color = Color {
        red: 0.0,
        green: 0.5,
        blue: 0.5,
    };

    pub const OLIVE: Color = Color {
        red: 0.5,
        green: 0.5,
        blue: 0.0,
    };

    pub const GRAY: Color = Color {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    const MAX_SUBPIXEL_VALUE: f64 = 255.0;

    pub fn new<I: Into<f64>>(r: I, g: I, b: I) -> Self {
        Self {
            red: r.into(),
            green: g.into(),
            blue: b.into(),
        }
    }

    #[allow(clippy::all)]
    pub fn from_hex(hex_color: u32) -> Self {
        // Hex color format = 0xrrggbb
        let r = (hex_color >> (8 * 2) & 0xFF) as f64;
        let g = (hex_color >> (8 * 1) & 0xFF) as f64;
        let b = (hex_color >> (8 * 0) & 0xFF) as f64;
        Self::new(
            r / Self::MAX_SUBPIXEL_VALUE,
            g / Self::MAX_SUBPIXEL_VALUE,
            b / Self::MAX_SUBPIXEL_VALUE,
        )
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }

    fn normalize(subpixel: f64) -> u8 {
        let value = subpixel.clamp(0.0, 1.0) * Self::MAX_SUBPIXEL_VALUE;
        value.round() as u8
    }

    pub fn to_byte_triple(self) -> (u8, u8, u8) {
        (
            Self::normalize(self.red),
            Self::normalize(self.green),
            Self::normalize(self.blue),
        )
    }
}
