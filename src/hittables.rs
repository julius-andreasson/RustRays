#[allow(dead_code)]
pub mod hittables {
    use std::rc::Rc;

    use crate::types::interval::Interval;
    use crate::types::util;
use crate::types::vec3::Vec3;
    use crate::types::ray::Ray;

    #[derive(Clone, Copy)]
    pub struct HitRecord {
        pub p: Vec3,
        pub t: f64,
        pub mat: Material,
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
        
        pub fn new() -> Self {
            Self { p: Vec3::default(), t: 0.0, mat: Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)}, normal: Vec3::default(), front_face: true }
        }
    }
    
    pub trait Hittable {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    }

    // HittableList
    pub struct HittableList <'a> {
        objects: Vec<Rc<&'a dyn Hittable>>
    }
    
    impl<'a> HittableList<'a> {
        pub fn new() -> Self {
            let objects: Vec<Rc<&'a dyn Hittable>> = vec![];
            Self { objects }
        }

        pub fn clear(&mut self) {
            self.objects.clear();
        }

        pub fn add(&mut self, object: &'a dyn Hittable) {
            let o: Rc<&'a dyn Hittable> = Rc::new(object);
            // let o = Rc::new(object);
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
                    *rec = *temp_rec; // ?? Hmm maybe there's a clever Rust feature to do this another way
                }
            }

            hit_anything
        }
    }

    // Shapes
    pub struct Sphere {
        center: Vec3,
        radius: f64,
        mat: Material
    }
    impl Sphere {
        pub fn new(center: Vec3, radius: f64, mat: Material) -> Self {
            Self { center: center, radius: radius, mat: mat }
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
            rec.mat = self.mat;

            return true;
        }
    }

    #[derive(Clone, Copy)]
    pub enum Material {
        Lambertian{albedo: Vec3},
        Metal{albedo: Vec3, fuzz: f64},
        Dielectric{refraction_index: f64}
    }

    pub fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, mat: Material) -> bool {
        match mat {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = rec.normal + Vec3::random_unit_vec();

                // Catch degenerate scatter direction $
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }

                let ray = Ray::new(rec.p, scatter_dir);
                *scattered = ray;
                *attenuation = albedo;
                // *attenuation = albedo.clone(); // $U$
                return true;
            },
            Material::Metal { albedo, fuzz } => {
                let checked_fuzz = if fuzz < 1.0 {fuzz} else {1.0};
                let mut reflected = Vec3::reflect(r_in.dir(), rec.normal);
                reflected = reflected.unit_vec() + (checked_fuzz * Vec3::random_unit_vec());
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = albedo;
                return Vec3::dot(&scattered.dir(), rec.normal) > 0.0;
            },
            Material::Dielectric { refraction_index } => {
                let ri = if rec.front_face {1.0/refraction_index} else {refraction_index};
                let unit_dir = r_in.dir().unit_vec();
                
                let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let dir: Vec3;
                if cannot_refract || reflectance(cos_theta, ri) > util::rnd() {
                    dir = Vec3::reflect(unit_dir, rec.normal);
                } else {
                    dir = Vec3::refract(unit_dir, rec.normal, ri);
                }

                *scattered = Ray::new(rec.p, dir);
                *attenuation = Vec3::new(1.0, 1.0, 1.0);
                return true;
            },
        }
    }

    fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance. $
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0-r0)*(1.0-cos).powi(5);
    }
}