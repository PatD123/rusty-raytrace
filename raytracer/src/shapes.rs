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
}

impl HitRec {
    pub fn new() -> Self{
        Self {
            hit_p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: true,
        }
    }

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
        
        for obj in &self.objs {
            if obj.hit(r, tmin, closest, &mut temp_rec) {
                flag = true;
                closest = temp_rec.t;
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
    pub fn new(sphere_center: Vec3, radius: f32) -> Self {
        Self {
            center: sphere_center,
            radius: radius,
            color: Vec3::new(1.0, 1.0, 1.0),
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
        rec.set_face_normal(r, outward_normal);

        true
    }
}