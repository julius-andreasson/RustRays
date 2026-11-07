#[allow(dead_code)]
pub mod util {
    use rand::RngExt;

    // Constants
    pub const INF: f64 = f64::INFINITY;
    pub const PI: f64 = std::f64::consts::PI;

    // Utility Functions
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        return degrees * PI / 180.0;
    }

    pub fn rnd() -> f64 {
        // Is it a bad idea to create a new rand::rng each time? Probably!
        // Docs say that using a local bind avoids repeated init-checks.
        // ?? Perhaps I can move the initialization out, and pass a local rng var?
        rand::rng().random::<f64>()
    }

    pub fn rnd_range(min: f64, max: f64) -> f64 {
        min + (max-min)*rnd()
    }
}

#[allow(dead_code)]
pub mod interval {
    use crate::types::util;

    pub struct Interval {
        pub min: f64,
        pub max: f64
    }
    
    impl Interval {
        pub fn new(min: f64, max: f64) -> Self {
            Self { min, max }
        }

        pub fn empty() -> Interval {
            Interval::new(util::INF, -util::INF)
        }

        pub fn universe() -> Interval {
            Interval::new(-util::INF, util::INF)
        }
        
        pub fn size(&self) -> f64 {
            self.max - self.min
        }
        
        pub fn contains(&self, x: f64) -> bool {
            self.min <= x && x <= self.max
        }
        
        pub fn surrounds(&self, x: f64) -> bool {
            self.min < x && x < self.max
        }

        pub fn clamp(&self, x: f64) -> f64 {
            if x < self.min {
                return self.min
            } else if x > self.max {
                return self.max
            } else {
                return x
            }
        }
    }
}

pub mod ray {
    use crate::types::vec3::Vec3;

    #[derive(Clone, Copy, Default)]
    pub struct Ray {
        orig: Vec3,
        dir: Vec3
    }

    // Should it all be implemented using Clone?
    impl Ray {
        pub fn new(orig: Vec3, dir: Vec3) -> Ray {
            Ray {orig, dir}
        }
        
        pub fn orig(&self) -> Vec3 {
            self.orig.clone()
        }
        
        pub fn dir(&self) -> Vec3 {
            self.dir.clone()
        }

        pub fn at(&self, c: f64) -> Vec3 {
            self.orig.clone() + c * self.dir.clone()
        }
    }
}

#[allow(dead_code)]
pub mod vec3 {
    use std::ops;

    use crate::types::{interval::Interval, util::{rnd, rnd_range}};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    
    pub trait Color {
        fn rgb(&self) -> image::Rgb<u8>;
        fn new(x: f64, y: f64, z: f64) -> Vec3;
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        } else {
            return 0.0;
        }
    }

    impl Color for Vec3 {
        fn rgb(&self) -> image::Rgb<u8> {
            let r = linear_to_gamma(self.x);
            let g = linear_to_gamma(self.y);
            let b = linear_to_gamma(self.z);

            let intensity = Interval::new(0.0, 0.999); // ?? Could I save this value so I wouldn't need to reinitialize it for each pixel?
            let rbyte = (256.0 * intensity.clamp(r)) as u8;
            let gbyte = (256.0 * intensity.clamp(g)) as u8;
            let bbyte = (256.0 * intensity.clamp(b)) as u8;
            image::Rgb([rbyte, gbyte, bbyte])
        }
 
        fn new(x: f64, y: f64, z: f64) -> Vec3 {
            Vec3 {x, y, z}
        }
    }

    impl Vec3 {
        pub fn origin() -> Vec3 {
            Vec3 {x: 0.0, y: 0.0, z: 0.0}
        }

        pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
            Vec3 {x, y, z}
        }

        fn rnd() -> Vec3 {
            Vec3::new(rnd(), rnd(), rnd())
        }

        fn rnd_range(min: f64, max:f64) -> Vec3 {
            Vec3::new(rnd_range(min, max), rnd_range(min, max), rnd_range(min, max))
        }

        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
    
        pub fn length_squared(&self) -> f64 {
            self.x.abs()*self.x.abs() + self.y.abs()*self.y.abs() + self.z.abs()*self.z.abs()
        }

        pub fn near_zero(&self) -> bool {
            let s = 1e-8;
            return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
        }

        pub fn unit_vec(self) -> Vec3 {
            // Compute length first, to avoid borrowing to both division function and length function simultaneously.
            // ?? Is there a recommended way of doing this?
            let len = self.length();
            self / len
        }

        pub fn random_unit_vec() -> Vec3 {
            loop {
                let p = Vec3::rnd_range(-1.0, 1.0);
                let lensq = p.length_squared();
                if 1e-160 < lensq && lensq <= 1.0 { // hmm interesting solution. See 'Listing 49' in book. Would need to find the corresponding number for float in order to use that. @float
                    return p / lensq.sqrt();
                }
            }
        }

        pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
            let on_unit_sphere = Vec3::random_unit_vec();
            if on_unit_sphere.dot(normal) > 0.0 { // Is in the same hemisphere as the normal $
                return on_unit_sphere;
            } else {
                return -on_unit_sphere;
            }
        }

        pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
            v - 2.0*v.dot(n)*n
        }

        pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
            let cos_theta = (-uv).dot(n).min(1.0);
            let r_out_perp = etai_over_etat * (uv + cos_theta * n);
            let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt())*n;
            return r_out_perp + r_out_parallel;
        }

        pub fn dot(&self, rhs: Vec3) -> f64 {
            self.x * rhs.x + 
            self.y * rhs.y + 
            self.z * rhs.z 
        }

        pub fn cross(&self, rhs: Vec3) -> Vec3 {
            Vec3::new(
                self.y*rhs.z - self.z*rhs.y,
                self.z*rhs.x - self.x*rhs.z,
                self.x*rhs.y - self.y*rhs.x
            )
        }
    }

    impl PartialEq for Vec3 {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }
    
    impl ops::Neg for Vec3 {
        type Output = Vec3;

        fn neg(self) -> Self::Output {
            self * (-1.0)
        }
    }

    // Unimplemented:
    // []
    // &[]

    impl ops::AddAssign<Vec3> for Vec3 {
        fn add_assign(&mut self, rhs: Vec3) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl ops::MulAssign<Vec3> for Vec3 {
        fn mul_assign(&mut self, rhs: Vec3) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
        }
    }

    impl ops::DivAssign<Vec3> for Vec3 {
        fn div_assign(&mut self, rhs: Vec3) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
        }
    }
    
    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;
        
        fn add(self, rhs: Vec3) -> Self::Output {
            Vec3 { x: self.x+rhs.x, y: self.y+rhs.y, z: self.z+rhs.z }
        }
    }
    
    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;
        
        fn sub(self, rhs: Vec3) -> Self::Output {
            Vec3 { x: self.x-rhs.x, y: self.y-rhs.y, z: self.z-rhs.z }
        }
    }

    // Vec3 * Vec3
    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;
        
        fn mul(self, rhs: Vec3) -> Self::Output {
            Vec3 { x: self.x*rhs.x, y: self.y*rhs.y, z: self.z*rhs.z }
        }
    }

    // Vec3 * f64
    impl ops::Mul<f64> for Vec3 {
        type Output = Vec3;
        
        fn mul(self, rhs: f64) -> Self::Output {
            Vec3 { x: self.x*rhs, y: self.y*rhs, z: self.z*rhs }
        }
    }
    
    // f64 * Vec3
    impl ops::Mul<Vec3> for f64 {
        type Output = Vec3;
        
        fn mul(self, rhs: Vec3) -> Self::Output {
            // Just call "Vec3 * f64", which is already implemented.
            rhs * self
        }
    }

    impl ops::Div<f64> for Vec3 {
        type Output =  Vec3;

        fn div(self, rhs: f64) -> Self::Output {
            // Just invert the f64, and then call "Vec3 * f64".
            self * (1.0 / rhs)
        }
    }

}

#[cfg(test)]
mod ray_tests {
    use crate::types::ray::Ray;
    use crate::types::vec3::Vec3;

    #[test]
    fn test_orig() {
        let orig = Vec3 {x: 3.0, y: 4.0, z: 0.0};
        let dir = Vec3 {x: 2.0, y: 1.0, z: 5.0};
        let r = Ray::new(orig, dir);
        let expected = Vec3 {x: 3.0, y: 4.0, z: 0.0};
        assert_eq!(r.orig(), expected);
    }

    #[test]
    fn test_dir() {
        let orig = Vec3 {x: 3.0, y: 4.0, z: 0.0};
        let dir = Vec3 {x: 2.0, y: 1.0, z: 5.0};
        let r = Ray::new(orig, dir);
        let expected = Vec3 {x: 2.0, y: 1.0, z: 5.0};
        assert_eq!(r.dir(), expected);
    }

    #[test]
    fn test_at() {
        let orig = Vec3 {x: 3.0, y: 4.0, z: 0.0};
        let dir = Vec3 {x: 2.0, y: 1.0, z: 5.0};
        let c = 3.0;
        let r = Ray::new(orig, dir);
        let expected = Vec3 {x: 3.0 + 2.0 * 3.0, y: 4.0 + 1.0 * 3.0, z: 0.0 + 5.0 * 3.0};
        assert_eq!(r.at(c), expected);
    }
}

#[cfg(test)]
mod vec3_tests {
    use crate::Vec3;

    #[test]
    fn test_length() {
        let p = Vec3 {x: 3.0, y: 4.0, z: 0.0};
        let expected = 5.0;
        assert_eq!(p.length(), expected);
    }

    #[test]
    fn test_unit_vector() {
        let p = Vec3 {x: 3.0, y: 4.0, z: 0.0}.unit_vec();
        let expected = Vec3 {x: 6.0, y: 8.0, z: 0.0};
        assert_eq!((p.x * 10.0).round(), expected.x.round());
        assert_eq!((p.y * 10.0).round(), expected.y.round());
        assert_eq!((p.z * 10.0).round(), expected.z.round());
        // assert_eq!(p.unit_vec(), expected);
    }

    #[test]
    fn test_add_1() {
        let p1 = Vec3 {x: 1.0, y: 0.0, z: 2.0};
        let p2 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
        let res = p1 + p2;
        let expected = Vec3 {x: 1.0, y: 1.0, z: 2.0};
        assert_eq!(res, expected);
    }

    #[test]
    fn test_add_2() {
        let p1 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
        let p2 = Vec3 {x: -1.0, y: -1.0, z: 0.0};
        let res = p1 + p2;
        let expected = Vec3 {x: 0.0, y: -1.0, z: 0.0};
        assert_eq!(res, expected);
    }

    #[test]
    fn test_sub_1() {
        let p1 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
        let p2 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
        let res = p1 - p2;
        let expected = Vec3 {x: 1.0, y: -1.0, z: 0.0};
        assert_eq!(res, expected);
    }

    #[test]
    fn test_sub_2() {
        let p1 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
        let p2 = Vec3 {x: -1.0, y: -1.0, z: 0.0};
        let res = p1 - p2;
        let expected = Vec3 {x: 2.0, y: 1.0, z: 0.0};
        assert_eq!(res, expected);
    }

    #[test]
    fn test_mul_point_x_point() {
        let p1 = Vec3 {x: 2.0, y: 4.0, z: 1.0};
        let p2 = Vec3 {x: 3.0, y: 2.0, z: 2.0};
        let res = p1 * p2;
        let expected = Vec3 {x: 6.0, y: 8.0, z: 2.0};
        assert_eq!(res, expected);
    }

    #[test]
    fn test_mul_point_x_const() {
        let p = Vec3 {x: 2.0, y: 4.0, z: 1.0};
        let c = 3.0;
        let res = p * c;
        let expected = Vec3 {x: 6.0, y: 12.0, z: 3.0};
        assert_eq!(res, expected);
    }

    #[test]
    fn test_mul_const_x_point() {
        let p = Vec3 {x: 2.0, y: 4.0, z: 1.0};
        let c = 3.0;
        let res = c * p;
        let expected = Vec3 {x: 6.0, y: 12.0, z: 3.0};
        assert_eq!(res, expected);
    }

    // addassign
    #[test]
    fn test_addassign() {
        let mut p1 = Vec3 {x: 2.0, y: 4.0, z: 1.0};
        let p2 = Vec3 {x: 3.0, y: 2.0, z: 2.0};
        p1 += p2;
        let expected = Vec3 {x: 5.0, y: 6.0, z: 3.0};
        assert_eq!(p1, expected);
    }

    // mulassign
    #[test]
    fn test_mulassign() {
        let mut p1 = Vec3 {x: 2.0, y: 4.0, z: 1.0};
        let p2 = Vec3 {x: 3.0, y: 2.0, z: 2.0};
        p1 *= p2;
        let expected = Vec3 {x: 6.0, y: 8.0, z: 2.0};
        assert_eq!(p1, expected);
    }

}