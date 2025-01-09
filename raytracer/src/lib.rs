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
