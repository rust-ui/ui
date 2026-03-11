#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Oklch {
    pub l: f32, // Lightness
    pub c: f32, // Chroma
    pub h: f32, // Hue
}

impl Oklch {
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }

    pub fn to_oklch_string(&self) -> String {
        format!("oklch({} {} {})", self.l, self.c, self.h)
    }

    pub fn secondary_with_factor(&self, factor: f32) -> Self {
        Self { l: self.l * factor, c: self.c * factor, h: self.h * factor }
    }
}
