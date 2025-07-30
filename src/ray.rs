use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, time: f64) -> Vec3 {
        Vec3 {
            x: self.origin.x + (time * self.direction.x),
            y: self.origin.y + (time * self.direction.y),
            z: self.origin.z + (time * self.direction.z)
        }
    }
}
