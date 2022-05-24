use crate::Lab;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(non_snake_case)]
#[inline]
pub const fn Xyz(x: f64, y: f64, z: f64) -> Xyz {
    Xyz { x, y, z }
}

impl Xyz {
    pub fn to_lab(&self) -> Lab {
        fn map_channel(c: f64) -> f64 {
            if c > 0.008856 {
                c.powf(1.0 / 3.0)
            } else {
                (7.787 * c) + (16.0 / 116.0)
            }
        }
        let x = map_channel(self.x / 95.0470);
        let y = map_channel(self.y / 100.000);
        let z = map_channel(self.z / 108.883);
        Lab {
            l: (y * 116.0) - 16.0,
            a: (x - y) * 500.0,
            b: (y - z) * 200.0,
        }
    }
}
