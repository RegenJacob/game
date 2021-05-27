pub struct Color {
    red: i16,
    green: i16,
    blue: i16,
}

// Theming

/// in hex: #27292d
pub const BACKGROUND_COLOR: Color = Color::from_rgb(39, 41, 45);
/// in hex: #9ba4b5
pub const FOREGROUND_COLOR: Color = Color::from_rgb(155, 164, 181);

impl Color {
    pub const fn from_rgb(red: i16, green: i16, blue: i16) -> Self {
        Self { red, green, blue }
    }

    pub fn get_glfloat_red(&self) -> f32 {
        self.red as f32 / 255.0
    }

    pub fn get_glfloat_green(&self) -> f32 {
        self.green as f32 / 255.0
    }

    pub fn get_glfloat_blue(&self) -> f32 {
        self.blue as f32 / 255.0
    }
}
