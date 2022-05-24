use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[inline]
pub const fn Rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb { r, g, b }
}

impl Rgb {
    #[inline]
    pub const fn gray(c: u8) -> Self {
        Rgb { r: c, g: c, b: c }
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

impl const Default for Rgb {
    #[inline]
    fn default() -> Self {
        Rgb { r: 0, g: 0, b: 0 }
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
