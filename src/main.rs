use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    camera::Camera,
    util::{clamp, Color, Scene, Sphere},
};

pub mod camera;
pub mod tracing;
pub mod util;
pub mod material;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = (WIDTH as f64 / (16. / 9.)) as u32;
pub const SAMPLES: usize = 30;
pub const MAX_DEPTH: usize = 150;

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

    let mut pix = 0;

    let mut scene = Scene::new();

    scene.add(Box::new(Sphere::new((0., 0., -1.).into(), 0.5, material::Material::Lambertian { albedo: Color::new(1., 0., 0.) })));
    scene.add(Box::new(Sphere::new((1., 0., -1.).into(), 0.5, material::Material::Lambertian { albedo: Color::new(1., 0., 0.) })));
    scene.add(Box::new(Sphere::new((-1., 0., -1.).into(), 0.5, material::Material::Lambertian { albedo: Color::new(1., 0., 0.) })));
    scene.add(Box::new(Sphere::new((0., -100.5, -1.).into(), 100., material::Material::Lambertian { albedo: Color::new(0., 1., 0.) })));

    let camera = Camera::new();

    let total = HEIGHT * WIDTH;
    let mut last_percentage = 0.;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let current_percentage = (y * WIDTH + x) as f64 * 100. / total as f64;
            if current_percentage > last_percentage {
                println!("Generating frame: {}%", current_percentage);
            }

            last_percentage = current_percentage;

            let mut color = Color::new(0., 0., 0.);

            let mut rng = rand::thread_rng();

            for _ in 0..SAMPLES {
                let u = (x as f64 + rng.gen_range(0.0..1.0)) / (WIDTH - 1) as f64;
                let v = (y as f64 + rng.gen_range(0.0..1.0)) / (HEIGHT - 1) as f64;

                let ray = camera.ger_ray(u, v);
                color += ray.color(&scene, MAX_DEPTH);
            }

            let scale = 1. / SAMPLES as f64;
            color.x *= scale;
            color.y *= scale;
            color.z *= scale;

            color.x = clamp(color.x, 0.0, 0.999).sqrt();
            color.y = clamp(color.y, 0.0, 0.999).sqrt();
            color.z = clamp(color.z, 0.0, 0.999).sqrt();

            frame[(WIDTH * HEIGHT * 4 - 1) as usize - (y * WIDTH + x + pix + 3) as usize] =
                (color[0] * 256.) as u8;
            frame[(WIDTH * HEIGHT * 4 - 1) as usize - (y * WIDTH + x + pix + 2) as usize] =
                (color[1] * 256.) as u8;
            frame[(WIDTH * HEIGHT * 4 - 1) as usize - (y * WIDTH + x + pix + 1) as usize] =
                (color[2] * 256.) as u8;
            frame[(WIDTH * HEIGHT * 4 - 1) as usize - (y * WIDTH + x + pix) as usize] = 255;
            pix += 3;
        }
    }

    println!("Frame time: {}ms", (Instant::now() - start).as_millis());
}
