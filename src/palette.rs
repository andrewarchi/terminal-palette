use crate::{Lab, Point3, Rgb, Xyz};

pub struct Palette<T>(pub Vec<T>);

impl<T> Palette<T>
where
    T: Into<Point3> + Copy,
{
    pub fn nearest(&self, c: T) -> Option<usize> {
        let c = c.into();
        let mut nearest = None;
        let mut min_dist = f64::INFINITY;
        for (i, c2) in self.0.iter().enumerate() {
            let dist = c.dist((*c2).into());
            if dist < min_dist {
                nearest = Some(i);
                min_dist = dist;
            }
        }
        nearest
    }
}

impl Palette<Rgb> {
    pub fn to_xyz(&self) -> Palette<Xyz> {
        Palette(self.0.iter().map(|c| c.to_xyz()).collect())
    }
    pub fn to_lab(&self) -> Palette<Lab> {
        Palette(self.0.iter().map(|c| c.to_lab()).collect())
    }
}

impl Palette<Xyz> {
    pub fn to_lab(&self) -> Palette<Lab> {
        Palette(self.0.iter().map(|c| c.to_lab()).collect())
    }
}
