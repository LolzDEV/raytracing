use cgmath::{Point3, Vector3};

use crate::tracing::Ray;

pub struct Camera {
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    eye: Point3<f64>,
    lower_left_corner: Point3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_lenght = 1.;
        let origin = Point3::new(0., 0., 0.);

        let horizontal = Vector3::new(viewport_width, 0., 0.);
        let vertical = Vector3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., focal_lenght);

        Self {
            horizontal,
            vertical,
            eye: origin,
            lower_left_corner,
        }
    }

    pub fn ger_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.eye, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.eye)
    }
}
