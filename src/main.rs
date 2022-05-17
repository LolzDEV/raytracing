use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    camera::Camera,
    util::{clamp, Color, HittableObject, Scene},
};

pub mod camera;
pub mod material;
pub mod tracing;
pub mod util;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = (WIDTH as f64 / (16. / 9.)) as u32;
pub const SAMPLES: usize = 50;
pub const MAX_DEPTH: usize = 100;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Ray Tracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            render(pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

pub fn render(frame: &mut [u8]) {
    let start = Instant::now();

    let mut scene = Scene::new();

    scene.add(HittableObject::new_sphere(
        (0., 0., -1.).into(),
        0.5,
        material::Material::Lambertian {
            albedo: Color::new(0.7, 0.3, 0.3),
        },
    ));
    scene.add(HittableObject::new_sphere(
        (1., 0., -1.).into(),
        0.5,
        material::Material::Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzziness: 1.0,
        },
    ));
    scene.add(HittableObject::new_sphere(
        (-1., 0., -1.).into(),
        0.5,
        material::Material::Metal {
            albedo: Color::new(0.8, 0.8, 0.8),
            fuzziness: 0.3,
        },
    ));
    scene.add(HittableObject::new_sphere(
        (0., -100.5, -1.).into(),
        100.,
        material::Material::Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        },
    ));

let camera = Camera::new();

    for pixels in frame.chunks_exact_mut(4) {
        pixels[0] = 255;
        pixels[1] = 255;
        pixels[2] = 255;
        pixels[3] = 255;
    }

    frame
        .par_chunks_exact_mut(4)
        .rev()
        .enumerate()
        .for_each(|(i, pixels)| {
            let mut colors = vec![Color::new(0., 0., 0.); SAMPLES];

            colors.par_iter_mut().for_each_with(i, |i, c| {
                let x = *i % WIDTH as usize;
                let y = *i / WIDTH as usize;
                let mut rng = rand::thread_rng();
                let u = (x as f64 + rng.gen_range(0.0..1.0)) / (WIDTH - 1) as f64;
                let v = (y as f64 + rng.gen_range(0.0..1.0)) / (HEIGHT - 1) as f64;

                let ray = camera.ger_ray(u, v);
                *c += ray.color(&scene, MAX_DEPTH);
            });

            let mut color: Color = colors.par_iter().map(|x| x).sum();

            let scale = 1. / SAMPLES as f64;
            color.x *= scale;
            color.y *= scale;
            color.z *= scale;

            color.x = clamp(color.x, 0.0, 0.999).sqrt();
            color.y = clamp(color.y, 0.0, 0.999).sqrt();
            color.z = clamp(color.z, 0.0, 0.999).sqrt();

            pixels[0] = (color[0] * 256.) as u8;
            pixels[1] = (color[1] * 256.) as u8;
            pixels[2] = (color[2] * 256.) as u8;
            pixels[3] = 255;
        });

    println!("Frame time: {}ms", (Instant::now() - start).as_millis());
}
