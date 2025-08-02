use crate::colour::Colour;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hittable;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub colour: Colour,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> Option<Colour> {
        let offset: Vec3 = self.centre - ray.origin;
        // Need to solve for time = (-b +- sqrt(b^2 - 4ac))/ 2a
        let a = Vec3::dot(&ray.direction, ray.direction);
        let b = -2.0 * Vec3::dot(&ray.direction, offset);
        let c = Vec3::dot(&offset, offset) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c * 4.0);
        if discriminant < 0.0 {
            return None;
        }

        let time = (-b - discriminant.sqrt())/ (2.0 * a);
        if time <= 0.0 {
            return None;
        }

        // Calculate the colour
        let normal = Vec3::unit_vector(&(ray.at(time) - self.centre));
        let colour = Colour ( Vec3 {x: 1.0 + normal.x, y: 1.0 + normal.y, z: 1.0 + normal.z} ).0 * 0.5;

        Some(Colour (colour))
    }
}
