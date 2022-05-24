use crate::Point3;

pub struct Palette<T>(pub Vec<T>);

impl<T> Palette<T>
where
    T: Into<Point3> + Copy,
{
    pub fn nearest(&self, c: T) -> Option<T> {
        let c = c.into();
        let mut nearest = None;
        let mut min_dist = f64::INFINITY;
        for c2 in &self.0 {
            let dist = c.dist((*c2).into());
            if dist < min_dist {
                nearest = Some(*c2);
                min_dist = dist;
            }
        }
        nearest
    }
}
