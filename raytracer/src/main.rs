use raytracer::vec3;
use raytracer::ray;
use raytracer::shapes;
use raytracer::materials;

use vec3::Vec3;
use ray::Ray;
use shapes::World;
use shapes::Sphere;
use shapes::Triangle;
use raytracer::Camera;
use materials::{Lambertian, Metal};

use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 10;

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_left = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_right = Rc::new(Lambertian::new(Vec3::new(0.1, 0.7, 0.5)));
    let material_middle = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right_back = Rc::new(Metal::new(Vec3::new(0.5, 0.1, 0.1)));


    let mut world = World::new();
    // world.add_obj(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, 1.0), 0.5, material_left))); // Left
    world.add_obj(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, material_middle))); // Middle
    world.add_obj(Box::new(Sphere::new(Vec3::new(1.5, 0.0, 0.0), 0.5, material_right))); // Right
    world.add_obj(Box::new(Sphere::new(Vec3::new(1.5, 1.5, -4.0), 2.0, material_right_back))); // Right Back

    world.add_obj(Box::new(Sphere::new(Vec3::new(0.0, -260.5, 0.0), 260.0, material_ground)));


    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_left = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_right = Rc::new(Lambertian::new(Vec3::new(0.5, 0.2, 0.8)));
    let a = Vec3::new(-2.0, -0.5, 1.0);
    let b = Vec3::new(-1.0, -0.5, 1.0);
    let c = Vec3::new(-1.5, 3.0, 0.5);
    world.add_obj(Box::new(Triangle::new(a, b, c, material_left)));
    let a = Vec3::new(-1.0, -0.5, 1.0);
    let b = Vec3::new(-1.5, 3.0, 0.5);
    let c = Vec3::new(-1.0, -0.5, -0.0);
    world.add_obj(Box::new(Triangle::new(a, b, c, material_right)));
    let a = Vec3::new(-2.0, -0.5, 1.0);
    let b = Vec3::new(-1.5, 3.0, 0.5);
    let c = Vec3::new(-1.0, -0.5, 0.0);
    world.add_obj(Box::new(Triangle::new(a, b, c, material_ground)));

    camera.initialize();
    camera.animate(&world);
}

// TODO
// Look at RAYON
// Look at SIMD

// Multithreading
// All the Rc should be Arcs
// Implement send and sync for shapes and shit.

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
    
    // #[test]
    // fn testing_random_vec() {
    //     let v = raytracer::random_vector();
    //     let v_= v.unit_vec();
    //     assert_eq!(v_.length(), 1.0);
    // }
}
