use std::fmt::Debug;

use crate::{
    tracing::Ray,
    util::{random_vector, unit_vector, Color, HitRecord, near_zero},
};

pub struct MaterialResult {
    pub hit: HitRecord,
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl Material {
    pub fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<MaterialResult> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit.normal + unit_vector(random_vector(0., 1.));

                if near_zero(scatter_direction) {
                    scatter_direction = hit.normal;
                }

                let scattered = Ray::new(hit.p, scatter_direction);
                Some(MaterialResult {
                    hit,
                    attenuation: *albedo,
                    scattered,
                })
            }
            Material::Metal { albedo } => todo!(),
        }
    }
}
