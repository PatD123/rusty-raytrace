use std::fs::File;
use std::io::Write;

use raytracer::vec3;
use raytracer::ray;
use raytracer::shapes;

use vec3::Vec3;
use ray::Ray;
use shapes::World;
use shapes::Sphere;
use raytracer::Camera;

fn main() {
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    let mut world = World::new();
    world.add_obj(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Vec3::new(0.0, 1.0, 0.0))));
    world.add_obj(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Vec3::new(0.0, 0.0, 1.0))));

    camera.render(&world);
}

// TODO
// Abstraction for hittable trait using Rust
// Make all shapes hittable.
// Make a Hittable list World to store hittable objects
// Figure out normals.
// Interval class.

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn basics() {
    //     let u = Vec3::new(0.0, 1.0, 2.0);
    //     let v = Vec3::new(3.0, 4.0, 5.0);

    //     let u_len = u.length();
    //     let v_len = v.length();
    //     assert_eq!(u_len, (5.0 as f32).sqrt());

    //     assert_eq!(v.unit_vec().x, 3.0/v_len);
    // }

    // #[test]
    // fn dot_and_cross() {
    //     let u = Vec3::new(0.0, 1.0, 2.0);
    //     let v = Vec3::new(3.0, 4.0, 5.0);

    //     assert_eq!(vec3::dot(u, v), 14.0);
    //     assert_eq!(vec3::dot(u, v), 14.0);

    //     assert_eq!(vec3::cross(&u, &v), Vec3::new(-3.0, 6.0, -3.0));
    // }

    // #[test]
    // fn testing_adds() {
    //     let mut u = Vec3::new(0.0, 1.0, 2.0);
    //     let v = Vec3::new(3.0, 4.0, 5.0);

    //     u += v;
    //     assert_eq!(u, Vec3::new(3.0, 5.0, 7.0));
        
    //     u += v + v;
    //     assert_eq!(u, Vec3::new(9.0, 13.0, 17.0));

    //     let n = v + v;
    //     assert_eq!(n, Vec3::new(6.0, 8.0, 10.0));
    // }

    // #[test]
    // fn testing_subs() {
    //     let u = Vec3::new(0.0, 1.0, 2.0);
    //     let p = Vec3::ZERO;
    //     let mut v = Vec3::new(3.0, 4.0, 5.0);

    //     v -= u;
    //     assert_eq!(v, Vec3::new(3.0, 3.0, 3.0));
        
    //     v -= u - p;
    //     assert_eq!(v, Vec3::new(3.0, 2.0, 1.0));
    // }

    // #[test]
    // fn testing_muls() {
    //     let u = Vec3::new(0.0, 1.0, 2.0);
    //     let v = Vec3::new(0.0, 1.0, 2.0);

    //     let mut p = u * v * 2.0;
    //     assert_eq!(p, Vec3::new(0.0, 2.0, 8.0));

    //     p *= 2.0;
    //     assert_eq!(p, Vec3::new(0.0, 4.0, 16.0));   
    // }

    // #[test]
    // fn testing_divs() {
    //     let mut u = Vec3::new(3.0, 4.0, 5.0);
    //     let v = Vec3::ONE;

    //     u /= 2.0;
    //     assert_eq!(u, Vec3::new(1.5, 2.0, 2.5));  
    // }

    // #[test]
    // fn testing_unary() {
    //     let u = Vec3::new(3.0, 4.0, 5.0);
    //     assert_eq!(-u, Vec3::new(-3.0, -4.0, -5.0));  
    // }

    // #[test]
    // fn testing_rays() {
    //     let origin = Vec3::ZERO;
    //     let direction = Vec3::new(0.0, 1.0, 2.0);
    //     let new_ray = Ray::new(origin, direction);

    //     assert_eq!(new_ray.origin(), Vec3::new(0.0, 0.0, 0.0));

    //     let dir = new_ray.direction() + Vec3::ONE;
    //     assert_eq!(dir, Vec3::new(1.0, 2.0, 3.0));

    //     assert_eq!(new_ray.at(5.0), Vec3::new(0.0, 5.0, 10.0));
    //     assert_eq!(new_ray.origin(), Vec3::new(0.0, 0.0, 0.0));
    // }

    // #[test]
    // fn basic_write_to_ppm() {
    //     let img_width = 256.0;
    //     let img_height = 256.0;
    //     let mut f = File::create("examples/output.ppm").expect("Couldn't create file!");

    //     let buf = ["P3\n", &img_width.to_string(), &format!(" {}\n", img_height.to_string()), "255\n"];
    //     for s in buf.iter() {
    //         f.write(s.as_bytes());
    //     }

    //     for i in 0..img_height as i32 {
    //         println!("Scanlines remaining: {}", (img_height as i32 - i));
    //         for j in 0..img_width as i32{
    //             let pixel_color = Vec3::new(j as f32 / (img_width - 1.0), i as f32 / (img_height - 1.0), 0.0);
    //             write_color(&f, &pixel_color);
    //         }
    //     }
    // }
}
