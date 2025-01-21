pub mod vec3;
pub mod ray;
pub mod shapes;
pub mod materials;

use vec3::Vec3;
use ray::Ray;
use shapes::World;
use shapes::HitRec;
use shapes::Hittable;
use materials::{Material, Lambertian};

use std::fs::File;
use std::io::Write;
use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
const ORIGIN: Vec3 = Vec3::ZERO;
const MAX_DEPTH: i32 = 10;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub focal_length: f32, 
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    
    image_height: i32,
    center: Vec3, 
    pixel_upper_left: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() ->  Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0,
            focal_length: 1.0,
            samples_per_pixel: 10,
            max_depth: 10,

            image_height: 0,
            center: Vec3::new(0.0, 0.0, 3.0),
            pixel_upper_left: Vec3::ZERO,
            pixel_delta_u: Vec3::ZERO,
            pixel_delta_v: Vec3::ZERO,
        }
    }

    pub fn initialize(&mut self) {
        // About image
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;

        // About camera: Camera orthogonal to viewport and points
        // directly in the middle of the viewport.
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // About viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // About individual pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f32);
        self.pixel_delta_v = viewport_v / (self.image_height as f32);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, self.focal_length);
        self.pixel_upper_left = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    pub fn animate(&mut self, world: &World) {
        for i in 0..360 {
            println!("Angles remaining: {}", (360 - i));
            let angle = -deg2rad(i as f32);
            self.rotate_y(angle);

            self.render_frame(world, i);

            self.rotate_y(-angle);

            break;
        }
    }

    pub fn render_frame(&mut self, world: &World, frame_id: i32) {
        // Render
        let nm = format!("testing/output{:03}.ppm", frame_id);
        let mut f = File::create(nm).expect("Couldn't create file!");
        let buf = ["P3\n", &self.image_width.to_string(), &format!(" {}\n", self.image_height.to_string()), "255\n"];
        for s in buf.iter() {
            f.write(s.as_bytes());
        }

        for i in 0..self.image_height {
            // println!("Scanlines remaining: {}", (self.image_height as i32 - i));
            for j in 0..self.image_width {
                // Used later to average for antialiasing
                let mut total_pixel_color = Vec3::ZERO;

                for k in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f32, j as f32);
                    let pixel_color = ray_color(&r, &world, MAX_DEPTH);
                    total_pixel_color += pixel_color;
                }

                total_pixel_color /= self.samples_per_pixel as f32; 

                write_color(&f, &total_pixel_color);
            }
        }
    }

    fn get_ray(&self, i: f32, j: f32) -> Ray {
        // Prolly should get a random x and random y offset
        let sampled_square_delta = sample_square();

        // Add to pixel_center to get the actual sample
        let mut pixel_center = self.pixel_upper_left + 
                               (self.pixel_delta_u * (j + sampled_square_delta.0)) +
                               (self.pixel_delta_v * (i + sampled_square_delta.1));

        // Return ray
        let ray_dir = pixel_center - self.center;
        Ray::new(self.center, ray_dir)
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.center.rotate_y(angle);
        self.pixel_upper_left.rotate_y(angle);
        self.pixel_delta_u.rotate_y(angle);
        self.pixel_delta_v.rotate_y(angle);
    }
}

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

pub fn ray_color(ray: &Ray, world: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::ONE;
    }

    let mut hit_rec = HitRec::new();
    if world.hit(ray, 0.001, INFINITY, &mut hit_rec) { // Dec for anti-acne.   
        // let c = (hit_rec.color + hit_rec.normal) * 0.5;
        // if c.x > 1.0 || c.y > 1.0 || c.z > 1.0 {
        //     return hit_rec.color;
        // }
        // return c;        

        // let refl = random_on_hemisphere(hit_rec.normal);
        // let refl = hit_rec.normal + random_vector(-1.0, 1.0).unit_vec();

        let mut r_scat = Ray::new(Vec3::ZERO, Vec3::ZERO);
        let mut atten = Vec3::ZERO;
        if hit_rec.mat.scatter(&ray, &hit_rec, &mut r_scat, &mut atten) {
            return atten * ray_color(&r_scat, world, depth - 1);
        }
        return Vec3::ZERO;
    }

    let unit_dir = ray.direction().unit_vec();
    let a = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

pub fn deg2rad(angle: f32) -> f32 {
    angle * std::f32::consts::PI/ 180.0
}

pub fn random_num(l: f32, h: f32) -> f32 {
    rand::thread_rng().gen_range(l..=h)
}

pub fn sample_square() -> (f32, f32) {
    (random_num(-0.5, 0.5), random_num(-0.5, 0.5))
}

pub fn clamp(min: f32, max: f32, num: f32) -> f32 {
    if num < min {
        min
    }
    else if num > max {
        max
    }
    else {
        num
    }
}

pub fn random_vector(l: f32, h: f32) -> Vec3 {
    Vec3 {
        x: random_num(l, h),
        y: random_num(l, h),
        z: random_num(l, h),
    }
}

pub fn random_on_hemisphere(n: Vec3) -> Vec3 {
    let v = random_vector(-1.0, 1.0).unit_vec();
    if vec3::dot(v, n) < 0.0 {
        -v
    }
    else {
        v
    }
}

pub fn reflect_vector(v: Vec3, n: Vec3) -> Vec3 {
    // Draw v twice in diagonal, then come up in diagram.
    v - n * 2.0 * vec3::dot(v, n)
}

// pub fn random_unit_vector() -> Vec3 {
//     loop {
//         let p = random_vector(-1.0, 1.0);
//         let lensq = p.length_squared();
//         println!("{}", lensq);
//         if f32::MIN < lensq && lensq <= 1.0 {
//             return p.unit_vec();
//         }
//     }
// }

// pub fn hit_sphere(sphere_center: Vec3, radius: f32, r: &Ray) -> f32{
//     let oc = sphere_center - r.origin();
//     let a = vec3::dot(r.direction(), r.direction());
//     let b = -2.0 * vec3::dot(r.direction(), oc);
//     let c = vec3::dot(oc, oc) - radius * radius;
//     let discriminant = b * b - 4.0 * a * c;
    
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (-b - discriminant.sqrt()) / (2.0 * a);
//     }
// }


