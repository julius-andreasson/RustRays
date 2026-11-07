#[allow(dead_code)]
pub mod hittables {
    use std::rc::Rc;

    use crate::types::interval::Interval;
    use crate::types::vec3::Vec3;
    use crate::types::ray::Ray;

    #[derive(Clone, Copy)]
    pub struct HitRecord {
        pub p: Vec3,
        pub t: f64,
        pub normal: Vec3,
        pub front_face: bool
    }

    impl HitRecord {
        fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
            // Sets the hit record normal vector. $
            // NOTE: the parameter `outward_normal` is assumed to have unit length. $

            self.front_face = r.dir().dot(outward_normal) < 0.0;
            self.normal = if self.front_face {outward_normal} else {-outward_normal}
        }
        
        fn set_to(&mut self, temp_rec: HitRecord) {
            self.p = temp_rec.p;
            self.t = temp_rec.t;
            self.normal = temp_rec.normal;
            self.front_face = temp_rec.front_face;
        }
        
        pub fn new() -> Self {
            Self { p: Vec3::origin(), t: 0.0, normal: Vec3::origin(), front_face: true }
        }
    }

    pub trait Hittable {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    }

    // HittableList
    pub struct HittableList<'a> {
        objects: Vec<Rc<&'a dyn Hittable>>,
    }
    
    impl<'a> HittableList<'a> {
        pub fn new() -> Self {
            let objects: Vec<Rc<&dyn Hittable>> = vec![];
            Self { objects }
        }

        pub fn clear(&mut self) {
            self.objects.clear();
        }

        pub fn add(&mut self, object: &'a dyn Hittable) {
            let o = Rc::new(object);
            self.objects.push(o); // Inline variable later
        }
    }

    impl<'a> Hittable for HittableList<'a> {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
            let temp_rec: &mut HitRecord = &mut rec.clone();
            let mut hit_anything= false;
            let mut closest_so_far = ray_t.max;

            for object in self.objects.iter() {
                if object.hit(r, Interval::new(ray_t.min, closest_so_far), temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    rec.set_to(*temp_rec); // Hmm
                }
            }

            hit_anything
        }
    }

    // Shapes
    pub struct Sphere {
        center: Vec3,
        radius: f64
    }
    impl Sphere {
        pub(crate) fn new(center: Vec3, radius: f64) -> Self {
            Self { center: center, radius: radius }
        }
    }

    impl Hittable for Sphere {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
            let oc = self.center - r.orig();
            let a = r.dir().length_squared();
            let h = r.dir().dot(oc);
            let c = oc.length_squared() - self.radius * self.radius;

            let discriminant = h*h - a*c;
            if discriminant < 0.0 {
                return false; 
            }
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range $
            let mut root = (h - sqrtd) / a;
            if !ray_t.contains(root) {
                root = (h + sqrtd) / a;
                if !ray_t.contains(root) {
                    return false;
                }
            }

            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, outward_normal);

            return true;
        }
    }
}