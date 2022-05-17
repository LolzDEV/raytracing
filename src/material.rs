use std::fmt::Debug;

use crate::{
    tracing::Ray,
    util::{near_zero, random_vector, reflect, unit_vector, Color, HitRecord, random_vector_in_unit_sphere},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialResult {
    pub hit: HitRecord,
    pub attenuation: Color,
    pub scattered: Ray,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzziness: f64 },
}

impl Material {
    pub fn scatter(self, ray: Ray, hit: HitRecord) -> Option<MaterialResult> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit.normal + unit_vector(random_vector(0., 1.));

                if near_zero(scatter_direction) {
                    scatter_direction = hit.normal;
                }

                let scattered = Ray::new(hit.p, scatter_direction);
                Some(MaterialResult {
                    hit,
                    attenuation: albedo,
                    scattered,
                })
            }
            Material::Metal { albedo , fuzziness} => {
                let reflected = reflect(unit_vector(ray.direction()), hit.normal);
                let scattered = Ray::new(hit.p, reflected + fuzziness * random_vector_in_unit_sphere());

                Some(MaterialResult {
                    hit,
                    attenuation: albedo,
                    scattered,
                })
            }
        }
    }
}
