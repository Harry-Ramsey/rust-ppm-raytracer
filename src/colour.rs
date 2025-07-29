use std::fmt;
use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq)]
pub struct Colour(pub Vec3);

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self.0 accesses the inner Vec3
        let r = (255.999 * self.0.x) as i32;
        let g = (255.999 * self.0.y) as i32;
        let b = (255.999 * self.0.z) as i32;
        write!(f, "{} {} {}", r, g, b)
    }
}
