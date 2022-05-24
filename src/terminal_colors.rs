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

// https://github.com/xtermjs/xterm.js/blob/master/src/browser/ColorManager.ts
pub const XTERM_ANSI_16: [Rgb; 16] = [
    Rgb::from(0x2E3436), // Black
    Rgb::from(0xCC0000), // Red
    Rgb::from(0x4E9A06), // Green
    Rgb::from(0xC4A000), // Yellow
    Rgb::from(0x3465A4), // Blue
    Rgb::from(0x75507B), // Magenta
    Rgb::from(0x06989A), // Cyan
    Rgb::from(0xD3D7CF), // White
    Rgb::from(0x555753), // Bright black
    Rgb::from(0xEF2929), // Bright red
    Rgb::from(0x8AE234), // Bright green
    Rgb::from(0xFCE94F), // Bright yellow
    Rgb::from(0x729FCF), // Bright blue
    Rgb::from(0xAD7FA8), // Bright magenta
    Rgb::from(0x34E2E2), // Bright cyan
    Rgb::from(0xEEEEEC), // Bright white
];

pub fn xterm_ansi_256() -> [Rgb; 256] {
    let mut colors = [Rgb::default(); 256];
    colors[..16].copy_from_slice(&XTERM_ANSI_16);

    // Fill in the remaining 240 ANSI colors.
    // Generate colors (16-231)
    let v = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];
    for i in 0..216 {
        colors[i + 16] = Rgb {
            r: v[(i / 36) % 6],
            g: v[(i / 6) % 6],
            b: v[i % 6],
        };
    }

    // Generate greys (232-255)
    for i in 0..24 {
        colors[i + 232] = Rgb::gray((i as u8) * 10 + 8);
    }
    colors
}
