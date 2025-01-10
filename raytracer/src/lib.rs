pub mod vec3;
pub mod ray;

pub use vec3::Vec3;
pub use ray::Ray;

use std::fs::File;
use std::io::Write;

pub fn write_color(mut f: &File, color: &Vec3) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    f.write(rbyte.to_string().as_bytes()); f.write(" ".as_bytes());
    f.write(gbyte.to_string().as_bytes()); f.write(" ".as_bytes());
    f.write(bbyte.to_string().as_bytes()); f.write("\n".as_bytes());
}

pub fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_dir = ray.direction().unit_vec();
    let a = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

pub fn hit_sphere(sphere_center: &Vec3, radius: f32, r: &Ray) -> bool{
    let oc = *sphere_center - r.origin();
    let a = vec3::dot(r.direction(), r.direction());
    let b = -2.0 * vec3::dot(r.direction(), oc);
    let c = vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    
    discriminant >= 0.0
}
