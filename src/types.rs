pub mod vec3 {
    use std::ops;

    #[derive(Debug)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Default for Vec3 {
        fn default() -> Self {
            Vec3::origin()
        }
    }

    impl Vec3 {
        pub fn origin() -> Vec3 {
            Vec3 {x: 0.0, y: 0.0, z: 0.0}
        }
        pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
            Vec3 {x, y, z}
        }

        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
    
        pub fn length_squared(&self) -> f64 {
            self.x.abs()*self.x.abs() + self.y.abs()*self.y.abs() + self.z.abs()*self.z.abs()
        }

        pub fn unit_vec(self) -> Vec3 {
            // Compute length first, to avoid borrowing to both division function and length function simultaneously.
            // What is the recommended way of doing this?
            // Surely not cloning to every call. That would be terribly inefficient.
            // And certainly not unsafely borrowing to multiple separately.
            let len = self.length();
            self / len
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