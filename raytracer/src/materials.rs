use crate::Ray;
use crate::HitRec;
use crate::Vec3;
use crate::random_vector;
use crate::reflect_vector;

// TODO
// Make material trait
// Lambertian
// Metal

// Pseudocode
// loop thru each obj in world -> you get a hit --> hit_rec
//      for the hit_rec (mat, hit_p)
//          1) Scatter off hit_rec.mat
//          2) Return scattered and attentuation from hit_p
// 
//       ray_color() recurse using atten_i, scattered_i
// 
// 

pub trait Material {
    fn scatter(&self, r: &Ray, hit_rec: &HitRec, r_scat: &mut Ray, atten: &mut Vec3) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: albedo,
        }
    }
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

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_rec: &HitRec, r_refl: &mut Ray, atten: &mut Vec3) -> bool {
        
        // Lambertian reflection
        let refl = reflect_vector(r.dir, hit_rec.normal);
        r_refl.o = hit_rec.hit_p;
        r_refl.dir = refl;
        *atten = self.albedo;
        
        true
    }
}


