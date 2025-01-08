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
        assert_eq!(u_len, (5.0 as f32).sqrt());

        assert_eq!(v.unit_vec().x, 3.0/v_len);
    }

    #[test]
    fn dot_and_cross() {
        let u = Vec3::new(0.0, 1.0, 2.0);
        let v = Vec3::new(3.0, 4.0, 5.0);

        assert_eq!(vec3::dot(&u, &v), 14.0);

        assert_eq!(vec3::cross(&u, &v), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn testing_adds() {
        let mut u = Vec3::new(0.0, 1.0, 2.0);
        let v = Vec3::new(3.0, 4.0, 5.0);

        u += v;
        assert_eq!(u, Vec3::new(3.0, 5.0, 7.0));
        
        u += v + v;
        assert_eq!(u, Vec3::new(9.0, 13.0, 17.0));

        let n = v + v;
        assert_eq!(n, Vec3::new(6.0, 8.0, 10.0));
    }

    #[test]
    fn testing_subs() {
        let u = Vec3::new(0.0, 1.0, 2.0);
        let p = Vec3::ZERO;
        let mut v = Vec3::new(3.0, 4.0, 5.0);

        v -= u;
        assert_eq!(v, Vec3::new(3.0, 3.0, 3.0));
        
        v -= u - p;
        assert_eq!(v, Vec3::new(3.0, 2.0, 1.0));
    }

    #[test]
    fn testing_muls() {
        let u = Vec3::new(0.0, 1.0, 2.0);
        let v = Vec3::new(0.0, 1.0, 2.0);

        let mut p = u * v * 2.0;
        assert_eq!(p, Vec3::new(0.0, 2.0, 8.0));

        p *= 2.0;
        assert_eq!(p, Vec3::new(0.0, 4.0, 16.0));   
    }

    #[test]
    fn testing_divs() {
        let mut u = Vec3::new(3.0, 4.0, 5.0);
        let v = Vec3::ONE;

        u /= 2.0;
        assert_eq!(u, Vec3::new(1.5, 2.0, 2.5));  
    }

    #[test]
    fn testing_unary() {
        let u = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(-u, Vec3::new(-3.0, -4.0, -5.0));  
    }
}
