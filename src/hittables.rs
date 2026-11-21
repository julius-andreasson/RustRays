pub mod hittables {
    use std::cmp::Ordering;
    use std::sync::Arc;
    use crate::types::interval::Interval;
    use crate::types::ray::Ray;
    use crate::types::vec3::{Axis, Color, Vec3};
    use crate::util::util::{self, PI};
    use crate::texture::texture::{SolidColor, Tex};

    type A = Arc<dyn Hittable + Sync + Send>;

    #[derive(Clone)]
    pub struct HitRecord {
        pub p: Vec3,
        pub t: f64,
        pub u: f64,
        pub v: f64,
        pub mat: Material,
        pub normal: Vec3,
        pub front_face: bool
    }

    impl HitRecord {
        fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
            // Sets the hit record normal vector. $
            // NOTE: the parameter `outward_normal` is assumed to have unit length. $

            self.front_face = r.dir.dot(&outward_normal) < 0.0;
            self.normal = if self.front_face {outward_normal} else {-outward_normal}
        }
        
        pub fn new() -> Self {
            Self { p: Vec3::default(), t: 0.0, u: 0.0, v: 0.0, mat: Material::Lambertian{ tex: Material::tex_from_color(Vec3::new(0.5, 0.5, 0.5)) }, normal: Vec3::default(), front_face: true }
        }
        
        fn set_to(&mut self, other: &HitRecord) {
            self.p = other.p.clone();
            self.t = other.t;
            self.mat = other.mat.clone();
            self.normal = other.normal.clone();
            self.front_face = other.front_face;
        }
    }
    
    pub trait Hittable {
        fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
        fn get_bbox(&self) -> &AABB;
    }

    // HittableList
    #[derive(Clone)]
    pub struct HittableList {
        pub objects: Vec<A>,
        bbox: AABB,
    }
    
    impl HittableList {
        pub fn new() -> Self {
            let objects: Vec<A> = vec![];
            let bbox: AABB = AABB::empty();
            Self { objects, bbox }
        }

        pub fn clear(&mut self) {
            self.objects.clear();
        }

        pub fn add<T: Hittable + 'static + Sync + Send>(&mut self, object: T) {
            let bbox: AABB = object.get_bbox().clone();
            let o: A = Arc::new(object);
            self.objects.push(o);
            self.bbox = AABB::from_aabbs(&self.bbox, &bbox);
        }
    }

    impl Hittable for HittableList {
        fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
            let temp_rec: &mut HitRecord = &mut rec.clone();
            let mut hit_anything= false;
            let mut closest_so_far = ray_t.max;

            for object in self.objects.iter() { // HMM
                if object.hit(r, &Interval::new(ray_t.min, closest_so_far), temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    rec.set_to(&temp_rec);
                    // ?? Hmm maybe there's a clever Rust feature to do this another way
                }
            }

            hit_anything
        }

        fn get_bbox(&self) -> &AABB {
            &self.bbox
        }
    }

    // Shapes
    pub struct Sphere {
        center: Vec3,
        radius: f64,
        mat: Material,
        bbox: AABB
    }
    impl Sphere {
        pub fn new(center: Vec3, radius: f64, mat: Material) -> Self {
            let rvec = Vec3::new(radius, radius, radius);
            let bbox = AABB::from_points(&(&center-&rvec), &(&center+&rvec));
            Self { center: center, radius: radius, mat: mat, bbox }
        }

        pub fn get_sphere_uv(p: &Vec3, u: f64, v: f64) -> (f64, f64) {
            // p: a given point on the sphere of radius one, centered at the origin.    $
            // u: returned value [0,1] of angle around the Y axis from X=-1.            $
            // v: returned value [0,1] of angle from Y=-1 to Y=+1.                      $
            //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>       $
            //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>       $
            //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>       $
            let theta = (-&p.y).acos();
            let phi = (-&p.z).atan2(p.x)+PI;
            
            return (phi/(2.0*PI), theta / PI)
        }
    }

    impl Hittable for Sphere {
        fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
            let oc = &self.center - &r.orig;
            let a = r.dir.length_squared();
            let h = r.dir.dot(&oc);
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
            let outward_normal = (&rec.p - &self.center) / self.radius;
            (rec.u, rec.v) = Sphere::get_sphere_uv(&outward_normal, rec.u, rec.v);
            rec.set_face_normal(r, outward_normal);
            rec.mat = self.mat.clone();

            return true;
        }
        fn get_bbox(&self) -> &AABB {
            &self.bbox
        }
    }

    pub enum Shape2DVariant {
        Quad,
        Disk,
        Triangle,
    }
    pub struct Shape2D {
        q: Vec3,
        u: Vec3,
        v: Vec3,
        w: Vec3,
        mat: Material,
        bbox: AABB,
        norm: Vec3,
        d: f64,
        variant: Shape2DVariant,
    }
    impl Shape2D {
        pub fn from_center_rad_norm(center: Vec3, radius: f64, norm: &Vec3, variant: Shape2DVariant, mat: Material) -> Self {
            // let norm = norm.unit_vec(); // safety
            let vup: Vec3; // unsafe when VUP is parallel to NORM, so if NORM is almost Y, we set VUP to X instead.
            if norm.unit_vec().y > 0.99 {
                vup = Vec3::new(1.0, 0.0, 0.0);
            } else {
                vup = Vec3::new(0.0, 1.0, 0.0);
            }
            let u = vup.cross(&norm).unit_vec()*radius; // really, *2.0 for both U and V, but we postpone multiplication to calc Q.
            let v = norm.cross(&u).unit_vec()*radius;
            let q = &center - &u - &v;
            Self::new(q, 2.0*u, 2.0*v, variant, mat)
        }

        pub fn new(q: Vec3, u: Vec3, v: Vec3, variant: Shape2DVariant, mat: Material) -> Self {
            let n = u.cross(&v);
            let norm = n.unit_vec();
            let d = norm.dot(&q);
            let w = &n / n.dot(&n);

            let bbox_diag1 = AABB::from_points(&q, &(&q + &u + &v));
            let bbox_diag2 = AABB::from_points(&(&q + &u), &(&q + &v));
            let bbox = AABB::from_aabbs(&bbox_diag1, &bbox_diag2);

            Self {q, u, v, w, mat, bbox, norm, d, variant}
        }
        
        fn is_interior(a: f64, b: f64, rec: &mut HitRecord, variant: &Shape2DVariant) -> bool {
            // Given the hit point in plane coordinates, return false if it is outside the $
            // primitive, otherwise set the hit record UV coordinates and return true. $
            match variant {
                Shape2DVariant::Quad => {
                    let unit_interval = Interval::new(0.0, 1.0);

                    if !unit_interval.contains(a) || !unit_interval.contains(b) {
                        return false;
                    }

                    rec.u = a;
                    rec.v = b;
                    return true;
                },
                Shape2DVariant::Disk => {
                    let ia = a - 0.5;
                    let ib = b - 0.5;

                    if (ia*ia+ib*ib).sqrt() > 0.5 {
                        return false;
                    }

                    rec.u = a;
                    rec.v = b;
                    return true;
                },
                Shape2DVariant::Triangle => {
                    if !(a > 0.0 && b > 0.0 && a + b < 1.0) {
                        return false;
                    }

                    rec.u = a;
                    rec.v = b;
                    return true;
                },
            }
        }
    }
    impl Hittable for Shape2D {
        fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
            let denom = &self.norm.dot(&r.dir);
            // No hit if the ray is parallel to the plane. $
            if denom.abs() < 1e-8 {
                return false;
            }
            // Return false if the hit point parameter t is outside the ray interval. $
            let t = (&self.d - &self.norm.dot(&r.orig)) / denom;
            if !ray_t.contains(t) {
                return false;
            }

            let intersection = r.at(t);
            let planar_hitpt_vec = &intersection - &self.q;
            let alpha = self.w.dot(&planar_hitpt_vec.cross(&self.v));
            let beta = self.w.dot(&self.u.cross(&planar_hitpt_vec));

            if !Shape2D::is_interior(alpha, beta, rec, &self.variant) {
                return false;
            }

            rec.t = t;
            rec.p = intersection;
            rec.mat = self.mat.clone();
            rec.set_face_normal(r, self.norm.clone());

            return true;
        }
    
        fn get_bbox(&self) -> &AABB {
            return &self.bbox
        }
    }

    #[derive(Clone)]
    pub enum Material {
        Lambertian{tex: Arc<Tex>},
        Metal{albedo: Vec3, fuzz: f64},
        Dielectric{refraction_index: f64},
        DiffuseLight{tex: Arc<Tex>},
    }
    impl Material {
        pub fn light_from_color(emit: Color) -> Self {
            Self::DiffuseLight { tex: Material::tex_from_color(emit) }
        }
        pub fn tex_from_color(new: Color) -> Arc<Tex> { // Weird placement of this tbh. §
            Arc::new(SolidColor::from_color(new))
        }
        pub fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
            match self {
                Material::DiffuseLight { tex } => {
                    tex.value(u, v, p)
                },
                _ => {
                    return Color::new(0.0, 0.0, 0.0);
                },
            }
        }
    }

    pub fn scatter(r_in: &Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match rec.mat {
            Material::Lambertian { tex } => {
                let pre_scatter_dir = &rec.normal + Vec3::random_unit_vec();
                // Catch degenerate scatter direction $
                let scatter_dir = if pre_scatter_dir.near_zero() {
                    rec.normal
                } else {
                    pre_scatter_dir
                };

                *attenuation = tex.value(rec.u, rec.v, &rec.p);
                let ray = Ray::new(rec.p, scatter_dir);
                *scattered = ray;
                return true;
            },
            Material::Metal { albedo, fuzz } => {
                let checked_fuzz = if fuzz < 1.0 {fuzz} else {1.0};
                let mut reflected = Vec3::reflect(&r_in.dir, &rec.normal);
                reflected = reflected.unit_vec() + (checked_fuzz * Vec3::random_unit_vec());
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = albedo;
                return Vec3::dot(&scattered.dir, &rec.normal) > 0.0;
            },
            Material::Dielectric { refraction_index} => {
                let ri = if rec.front_face {1.0/refraction_index} else {refraction_index};
                let unit_dir = Vec3::unit_vec(&r_in.dir);
                
                let cos_theta = (-&unit_dir).dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let dir: Vec3;
                dir = if cannot_refract || reflectance(cos_theta, ri) > util::rnd() {
                    Vec3::reflect(&unit_dir, &rec.normal)
                } else {
                    Vec3::refract(&unit_dir, &rec.normal, ri)
                };

                *scattered = Ray::new(rec.p, dir);
                *attenuation = Vec3::new(1.0, 1.0, 1.0);
                return true;
            },
            Material::DiffuseLight { tex: _ } => {
                return false;
            },
        }
    }

    fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance. $
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0-r0)*(1.0-cos).powi(5);
    }

    // AABB (Axis-Aligned Bounding Boxes)
    #[derive(Clone)]
    pub struct AABB {
        x: Interval,
        y: Interval,
        z: Interval,
    }

    impl AABB {
        pub fn empty() -> AABB {
            let x = Interval::empty();
            let y = Interval::empty();
            let z = Interval::empty();
            AABB { x, y, z }
        }

        pub fn universe() -> AABB {
            let x = Interval::universe();
            let y = Interval::universe();
            let z = Interval::universe();
            AABB { x, y, z }
        }

        pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> AABB {
            let mut res = AABB { x, y, z };
            res.pad_to_mins();
            return res;
        }
        
        pub fn from_points(a: &Vec3, b: &Vec3) -> AABB {
            // Treat the two points a and b as extrema for the bounding box, so we don't require a $
            // particular minimum/maximum coordinate order. $
            let x = if a.x <= b.x {Interval::new(a.x, b.x)} else {Interval::new(b.x, a.x)};
            let y = if a.y <= b.y {Interval::new(a.y, b.y)} else {Interval::new(b.y, a.y)};
            let z = if a.z <= b.z {Interval::new(a.z, b.z)} else {Interval::new(b.z, a.z)};
            let mut res = AABB { x, y, z };
            res.pad_to_mins();
            return res;
        }

        pub fn from_aabbs(a: &AABB, b: &AABB) -> AABB {
            let x = Interval::from_intervals(&a.x, &b.x);
            let y = Interval::from_intervals(&a.y, &b.y);
            let z = Interval::from_intervals(&a.z, &b.z);
            AABB { x, y, z }
        }

        pub fn axis_interval(&self, axis: &Axis) -> Interval {
            match axis {
                Axis::X => self.x.clone(),
                Axis::Y => self.y.clone(),
                Axis::Z => self.z.clone(),
            }
        }

        pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
            let ray_orig = &r.orig;
            let ray_dir = &r.dir;

            for axis in &[Axis::X, Axis::Y, Axis::Z] {
                let ax = self.axis_interval(axis);
                let adinv = 1.0 / ray_dir.get(axis);

                let t0 = (ax.min - ray_orig.get(axis)) * adinv;
                let t1 = (ax.max - ray_orig.get(axis)) * adinv;

                let mut min: f64 = ray_t.min;
                let mut max: f64 = ray_t.max;

                if t0 < t1 {
                    if t0 > min {min = t0;}
                    if t1 < max {max = t1;}
                } else {
                    if t1 > min {min = t1;}
                    if t0 < max {max = t0;}
                }

                if max <= min {return false}
            }

            return true
        }

        pub fn longest_axis(&self) -> Axis {
            // Returns the index of the longest axis of the bounding box. $
            if self.x.size() > self.y.size() {
                if self.x.size() > self.z.size() {
                    return Axis::X
                } else {
                    return Axis::Z
                }
            } else if self.y.size() > self.z.size() {
                return Axis::Y
            } else {
                return Axis::Z
            }
        }

        fn pad_to_mins(&mut self) {
            // Adjust the AABB so that no side is narrower than some delta, padding if necessary. $
            let delta: f64 = 0.0001;
            if self.x.size() < delta {self.x = self.x.expand(delta)}
            if self.y.size() < delta {self.y = self.y.expand(delta)}
            if self.z.size() < delta {self.z = self.z.expand(delta)}
        }
    }

    pub struct BVHNode {
        left: A,
        right: A,
        bbox: AABB,
    }

    impl Hittable for BVHNode {
        fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
            if !self.bbox.hit(r, ray_t) {
                return false;
            } else {
                let hit_left = self.left.hit(r, ray_t, rec);
                let hit_right = self.right.hit(r, &Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);
                return hit_left || hit_right;
            }
        }
    
        fn get_bbox(&self) -> &AABB {
            &self.bbox
        }

    }

    impl BVHNode {
        pub fn new(mut objects: Vec<A>) -> BVHNode {
            let left: A;
            let right: A;

            let object_span = objects.len();
            if object_span == 1 {
                left = objects.first().unwrap().clone();
                right = objects.first().unwrap().clone();
            } else if object_span == 2 {
                left = objects.first().unwrap().clone();
                right = objects.last().unwrap().clone();
            } else {
                // Build the bounding box of the span of source objects. $
                let mut bbox: AABB = AABB::empty();
                for o in objects.iter() {
                    bbox = AABB::from_aabbs(&bbox, o.get_bbox());
                }

                let axis: &Axis = &bbox.longest_axis();
                let comparator = |a: &A, b: &A| {
                    if a.get_bbox().axis_interval(axis).min < b.get_bbox().axis_interval(axis).min {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }.to_owned().to_owned()
                };

                objects.sort_by(comparator);
                let mid = object_span / 2;
                let (l, r) = objects.split_at_mut(mid);
                left = Arc::new(BVHNode::new(l.to_vec()));
                right = Arc::new(BVHNode::new(r.to_vec()));
            }
            let bbox = AABB::from_aabbs(left.get_bbox(), right.get_bbox());
            return BVHNode {left, right, bbox}
        }
    }
}