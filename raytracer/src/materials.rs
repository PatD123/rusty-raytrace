use crate::Ray;
use crate::HitRec;
use crate::Vec3;
use crate::random_vector;

// TODO
// Make material trait
// Lambertian
// Metal

pub trait Material {
    fn scatter(&self, r: &Ray, hit_rec: &HitRec, r_scat: &mut Ray, atten: &mut Vec3) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, hit_rec: &HitRec, r_scat: &mut Ray, atten: &mut Vec3) -> bool {
        
        // Lambertian reflection
        let refl = hit_rec.normal + random_vector(-1.0, 1.0).unit_vec();
        r_scat.o = hit_rec.hit_p;
        r_scat.dir = refl;
        *atten = self.albedo;
        
        true
    }
}
