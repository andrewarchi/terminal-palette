#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[inline]
pub const fn Rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb { r, g, b }
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

// https://github.com/xtermjs/xterm.js/blob/master/src/browser/ColorManager.ts
pub const XTERM_ANSI_COLORS: [Rgb; 16] = [
    0x2E3436.into(), // Black
    0xCC0000.into(), // Red
    0x4E9A06.into(), // Green
    0xC4A000.into(), // Yellow
    0x3465A4.into(), // Blue
    0x75507B.into(), // Magenta
    0x06989A.into(), // Cyan
    0xD3D7CF.into(), // White
    0x555753.into(), // Bright black
    0xEF2929.into(), // Bright red
    0x8AE234.into(), // Bright green
    0xFCE94F.into(), // Bright yellow
    0x729FCF.into(), // Bright blue
    0xAD7FA8.into(), // Bright magenta
    0x34E2E2.into(), // Bright cyan
    0xEEEEEC.into(), // Bright white
];

pub fn xterm_ansi_256() -> [Rgb; 256] {
    let mut colors = [Rgb::default(); 256];
    colors[..16].copy_from_slice(&XTERM_ANSI_COLORS);

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
        let c = (i as u8) * 10 + 8;
        colors[i + 232] = Rgb(c, c, c);
    }
    colors
}
