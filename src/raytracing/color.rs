use crate::raytracing::*;

pub trait ToColor8 {
    fn to_color8(&self) -> Color8;
}

impl ToColor8 for Color {
    fn to_color8(&self) -> Color8 {
        self.iter().map(|c| (c * 255.) as u8).collect::<Color8>()
    }
}

pub trait SrgbCorrectable {
    fn srgb(&self) -> Color;
}

impl SrgbCorrectable for Color {
    fn srgb(&self) -> Color {
        Color::from_iter(self.iter().map(|v| {
            if v <= 0.0031308 {
                v * 12.92
            } else {
                v.powf(1.0 / 2.4) * 1.055 - 0.055
            }
        }))
    }
}
