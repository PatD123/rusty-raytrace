use std::vec;

pub mod vec3;
pub mod ray;

pub use ray::Ray;
pub use vec3::Vec3;

pub trait Hittable {
    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32, hit_rec: &mut HitRec) -> bool;
}

pub struct HitRec {
    pub hit_p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRec {
    pub fn new() -> Self{
        Self {
            hit_p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0;
        self.normal = front_face ? outward_normal : -outward_normal;
    }

}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Vec3,
}

impl Sphere {
    pub fn new(sphere_center: &Vec3, radius: f32) -> Self {
        Self {
            center: sphere_center,
            radius: radius,
            color: Vec3::new(1, 1, 1),
        }
    }
}

impl Hittable for Sphere {
    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitRec) -> bool {
        let oc = self.center - r.origin();
        let a = vec3::dot(r.direction(), r.direction());
        let b = -2.0 * vec3::dot(r.direction(), oc);
        let c = vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return false;

        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
        if root < tmin || root > tmax {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root < tmin || root > tmax {
                return false;
            }
        }

        rec.t = root;
        rec.hit_p = r.at(root);
        let outward_normal = (rec.hit_p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}