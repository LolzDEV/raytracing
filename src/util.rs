use cgmath::{Point3, Vector3};
use rand::Rng;

use crate::{tracing::Ray, material::Material};

pub type Color = Vector3<f64>;

pub fn unit_vector(v: Vector3<f64>) -> Vector3<f64> {
    v / (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3<f64>,
    pub material: Material,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().x * ray.direction().x
            + ray.direction().y * ray.direction().y
            + ray.direction().z * ray.direction().z;
        let half_b = cgmath::dot(oc, ray.direction());
        let c = (oc.x * oc.x + oc.y * oc.y + oc.z * oc.z) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let front_face =
            cgmath::dot(ray.direction(), (ray.at(root) - self.center) / self.radius) < 0.;

        Some(HitRecord {
            p: ray.at(root),
            normal: if front_face {
                (ray.at(root) - self.center) / self.radius
            } else {
                -((ray.at(root) - self.center) / self.radius)
            },
            t: root,
            front_face,
            material: self.material
        })
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;

        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(found) = object.hit(ray, t_min, closest_so_far) {
                hit = Some(found);
                closest_so_far = found.t;
            }
        }

        hit
    }
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }

    x
}

pub fn random_vector(min: f64, max: f64) -> Vector3<f64> {
    let mut rng = rand::thread_rng();

    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_vector_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vector(-1., 1.);
        if p.x * p.x + p.y * p.y + p.z * p.z >= 1. {
            continue;
        }
        return p;
    }
}

pub fn near_zero(vec: Vector3<f64>) -> bool {
    let s = 1e-8;
    (vec[0].abs() < s) && (vec[1].abs() < s) && (vec[2].abs() < s)
}