use std::fs::File;
use std::io::Write;

use raytracer::Vec3;
use raytracer::Ray;
use raytracer::write_color;

fn main() {
    // About image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // About camera: Camera orthogonal to viewport and points
    // directly in the middle of the viewport.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let focal_length = 1.0;
    let camera = Vec3::ZERO;

    // About viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // About individual pixel
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel_upper_left = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    let mut f = File::create("examples/output.ppm").expect("Couldn't create file!");
    let buf = ["P3\n", &image_width.to_string(), &format!(" {}\n", image_height.to_string()), "255\n"];
    for s in buf.iter() {
        f.write(s.as_bytes());
    }

    for i in 0..image_height {
        println!("Scanlines remaining: {}", (image_height as i32 - i));
        for j in 0..image_width {
            let pixel_center = pixel_upper_left + (pixel_delta_u * j as f32) + (pixel_delta_v * i as f32);
            let ray_dir = pixel_center - camera;
            let r = Ray::new(camera, ray_dir);

            let pixel_color = raytracer::ray_color(&r);
            raytracer::write_color(&f, &pixel_color);
        }
    }
}

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

    //     assert_eq!(vec3::dot(&u, &v), 14.0);

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
