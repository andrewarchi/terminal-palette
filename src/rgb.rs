use crate::{Lab, Xyz};
use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(non_snake_case)]
#[inline]
pub const fn Rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb { r, g, b }
}

impl Rgb {
    #[inline]
    pub const fn gray(c: u8) -> Self {
        Rgb { r: c, g: c, b: c }
    }

    pub fn to_xyz(&self) -> Xyz {
        fn map_channel(c: u8) -> f64 {
            let norm_c = (c as f64) / 255.0;
            if norm_c > 0.04045 {
                ((norm_c + 0.055) / 1.055).powf(2.4) * 100.0
            } else {
                (norm_c / 12.92) * 100.0
            }
        }
        let r = map_channel(self.r);
        let g = map_channel(self.g);
        let b = map_channel(self.b);
        Xyz {
            x: (r * 0.4124) + (g * 0.3576) + (b * 0.1805),
            y: (r * 0.2126) + (g * 0.7152) + (b * 0.0722),
            z: (r * 0.0193) + (g * 0.1192) + (b * 0.9505),
        }
    }

    pub fn to_lab(&self) -> Lab {
        self.to_xyz().to_lab()
    }
}

impl const From<u32> for Rgb {
    #[inline]
    fn from(c: u32) -> Self {
        Rgb {
            r: (c >> 16) as u8,
            g: (c >> 8) as u8,
            b: c as u8,
        }
    }
}

impl const From<Rgb> for u32 {
    #[inline]
    fn from(c: Rgb) -> Self {
        (c.r as u32) << 16 | (c.g as u32) << 8 | (c.b as u32)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("#")?;
        }
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("#")?;
        }
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
