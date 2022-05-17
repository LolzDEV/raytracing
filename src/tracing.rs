use std::f64::INFINITY;

use cgmath::{Point3, Vector3};

use crate::util::{Color, Scene};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }

    pub fn color(self, scene: &Scene, depth: usize) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(hit) = scene.hit(self, 0.001, INFINITY) {
            if let Some(scattered) = hit.material.scatter(self, hit) {
                let scattered_color = scattered.scattered.color(scene, depth - 1);

                return Color::new(
                    scattered.attenuation.x * scattered_color.x,
                    scattered.attenuation.y * scattered_color.y,
                    scattered.attenuation.z * scattered_color.z,
                );
            }

            return Color::new(0., 0., 0.);
        }

        let unit = self.direction
            / (self.direction.x * self.direction.x
                + self.direction.y * self.direction.y
                + self.direction.z * self.direction.z)
                .sqrt();
        let t = 0.5 * (unit.y + 1.0);

        (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
}
