pub mod vec3;
pub mod ray;
pub mod shapes;

use vec3::Vec3;
use ray::Ray;
use shapes::World;
use shapes::HitRec;
use shapes::Hittable;

use std::fs::File;
use std::io::Write;

pub const INFINITY: f32 = f32::INFINITY;
const ORIGIN: Vec3 = Vec3::ZERO;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub focal_length: f32, 
    
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

            image_height: 0,
            center: Vec3::new(0.0, 0.0, 2.0),
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
            let angle = i as f32 * std::f32::consts::PI/ 180.0;
            let center = self.center;
            self.center.rotate_y(angle);
            let pixel_upper_left = self.pixel_upper_left;
            self.pixel_upper_left.rotate_y(angle);

            // println!("Before {}", self.center);

            self.render_frame(world, i);

            self.center = center;
            self.pixel_upper_left = pixel_upper_left;

            // println!("After {}", self.center.length());
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
                let pixel_center = self.pixel_upper_left + (self.pixel_delta_u * j as f32) + (self.pixel_delta_v * i as f32);
                let ray_dir = pixel_center - self.center;
                let r = Ray::new(self.center, ray_dir);

                let pixel_color = ray_color(&r, &world);
                write_color(&f, &pixel_color);
            }
        }
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

pub fn ray_color(ray: &Ray, world: &World) -> Vec3 {
    let mut hit_rec = HitRec::new();
    if world.hit(ray, 0.0, INFINITY, &mut hit_rec) {
        return hit_rec.color;
    }

    let unit_dir = ray.direction().unit_vec();
    let a = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

pub fn hit_sphere(sphere_center: &Vec3, radius: f32, r: &Ray) -> f32{
    let oc = *sphere_center - r.origin();
    let a = vec3::dot(r.direction(), r.direction());
    let b = -2.0 * vec3::dot(r.direction(), oc);
    let c = vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}
