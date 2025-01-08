mod vec3;

use vec3::Vec3;

fn main() {
    let v = Vec3::new(1.0, 1.0, 1.0);

    println!("{}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let u = Vec3::new(0.0, 1.0, 2.0);
        let v = Vec3::new(3.0, 4.0, 5.0);

        let u_len = u.length();
        let v_len = v.length();
        // Test magnitude
        assert_eq!(u_len, (5.0 as f32).sqrt());

        // Test vector normalization
        assert_eq!(v.unit_vec().x, 3.0/v_len);
    }

    #[test]
    fn dot_and_cross() {
        let u = Vec3::new(0.0, 1.0, 2.0);
        let v = Vec3::new(3.0, 4.0, 5.0);

        // Test dot product
        assert_eq!(vec3::dot(&u, &v), 14.0);

        // Test cross product
        assert_eq!(vec3::cross(&u, &v).y, 6.0);
    }
}
