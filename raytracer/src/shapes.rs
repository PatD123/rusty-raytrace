use std::vec;

use crate::Ray;
use crate::Vec3;
use crate::vec3;

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, hit_rec: &mut HitRec) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct HitRec {
    pub hit_p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub color: Vec3,
}

impl HitRec {
    pub fn new() -> Self{
        Self {
            hit_p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: true,
            color: Vec3::ZERO,
        }
    }

    // Determines if ray hits from front-face or back-face.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        }
        else {
            self.normal = -outward_normal;
        }
    }

}

pub struct World {
    objs: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, obj: Box<dyn Hittable>) {
        self.objs.push(obj);
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, hit_rec: &mut HitRec) -> bool {
        let mut temp_rec = HitRec::new();
        let mut flag = false;
        let mut closest = tmax;
        
        // Loop through all world objects
        for obj in &self.objs {

            // Have each object be hit (if it can get hit by ray)
            if obj.hit(r, tmin, closest, &mut temp_rec) {
                flag = true;
                closest = temp_rec.t; // Keep track of objects of least depth.
                *hit_rec = temp_rec;
            }
        }

        return flag;
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Vec3,
}

impl Sphere {
    pub fn new(sphere_center: Vec3, radius: f32, color: Vec3) -> Self {
        Self {
            center: sphere_center,
            radius: radius,
            color: color,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitRec) -> bool {
        let oc = self.center - r.origin();
        let a = vec3::dot(r.direction(), r.direction());
        let b = -2.0 * vec3::dot(r.direction(), oc);
        let c = vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return false;
        }

        // Determining valid t roots from sphere intersection
        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
        if root < tmin || root > tmax {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root < tmin || root > tmax {
                return false;
            }
        }

        // Recording it in the HitRecord.
        rec.t = root;
        rec.hit_p = r.at(root);
        rec.color = self.color;
        let outward_normal = (rec.hit_p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3, 
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            a: a,
            b: b,
            c: c,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitRec) -> bool {
        // N: normal of plane
        // R: Ray
        // if dot(R, N) is 0, then perp then parallel so no possible intersection.
        // If intersection is behind ray (-t), then we return false
        // The compute if inside triangle
    }
}