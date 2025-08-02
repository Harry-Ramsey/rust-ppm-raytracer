use crate::ray::Ray;
use crate::colour::Colour;

pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<Colour>;
}
