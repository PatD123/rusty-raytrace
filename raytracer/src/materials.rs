use crate::Ray;
use crate::HitRec;
use crate:Vec3;

// TODO
// Make material trait
// Lambertian
// Metal

pub trait Material {
    fn scatter(r: &Ray, hit_rec: &HitRec, r_scat: &mut Ray) -> bool;
}
