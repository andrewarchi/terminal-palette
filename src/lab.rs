#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[allow(non_snake_case)]
#[inline]
pub const fn Lab(l: f64, a: f64, b: f64) -> Lab {
    Lab { l, a, b }
}
