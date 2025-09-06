mod rl {
    pub use raylib::prelude::*;
}

mod utils;
mod camera;

use raylib::prelude::{RaylibDraw, RaylibShaderModeExt};
use camera::Camera;

const FOV: f32 = 50.0;
const NEAR_CLIP_DIST: f32 = 1.0;
// const FAR_CLIP_DIST: f32 = 100.0;

struct RayHit {
    position: rl::Vector3,
    normal: rl::Vector3,
    t: f32,
}

struct Sphere {
    center: rl::Vector3,
    radius: f32,
}

impl Sphere {
    fn hit(&self, ray: &rl::Ray) -> Option<RayHit> {
        let v = ray.position - self.center;
        let b = v.dot(ray.direction);
        let c = v.dot(v) - self.radius * self.radius;

        if c > 0.0 && b > 0.0 {
            return None;
        }

        let d = b * b - c;

        if d < 0.0 {
            return None;
        }

        let mut t = -b - d.sqrt();

        if t < 0.0 {
            t = 0.0;
        }

        let position = ray.position + ray.direction * t;
        let normal = (position - self.center) / self.radius;

        Some(RayHit { position, normal, t })
    }
}

fn main() {
    let (mut rl, rt) = raylib::init()
        .size(800, 600)
        .title("Hello, World")
        .build();

    rl.disable_cursor();

    let mut shader = rl.load_shader(
        &rt,
        None,
        Some("src/shaders/spheretrace.fs")
    );

    let mut camera = Camera::new(
        rl::Vector3::new(-0.7891882, 1.6806532, -2.4450927),
        rl::Vector3::new(0.17874563, -0.25690365, 0.25920606),
    );

    let sphere = Sphere {
        center: rl::Vector3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    };

    let mut framebuffer = vec![
        rl::Color::BLACK;
        (rl.get_screen_width() * rl.get_screen_height()) as usize
    ];

    while !rl.window_should_close() {
        camera.update_camera(&rl);

        let screen_width = rl.get_screen_width() as usize;
        let screen_height = rl.get_screen_height() as usize;

        // Screen plane stuff
        let sp_forward = camera.forward;
        let sp_right = camera.right;
        let sp_up = camera.up;

        let sp_half_height = (FOV * 0.5).tan() * NEAR_CLIP_DIST;
        let sp_half_width = sp_half_height * (screen_width as f32 / screen_height as f32);
        let sp_center = camera.position + sp_forward * NEAR_CLIP_DIST;
        let sp_top_left = sp_center + sp_up * sp_half_height - sp_right * sp_half_width;

        // for j in 0..screen_height {
        //     for i in 0..screen_width {
        //         // Normalized coordinates
        //         let u = i as f32 / (screen_width - 1) as f32;
        //         let v = j as f32 / (screen_height - 1) as f32;
        //
        //         // Pixel position on screen plane
        //         let pixel_pos = sp_top_left
        //             + sp_right * (u * 2.0 * sp_half_width)
        //             - sp_up * (v * 2.0 * sp_half_height);
        //         let ray = rl::Ray::new(
        //             camera.position,
        //             (pixel_pos - camera.position).normalized()
        //         );
        //
        //         framebuffer[j * screen_width + i] = if let Some(hit) = sphere.hit(&ray) {
        //             rl::Color::new(
        //                 (hit.normal.x * 127.5 + 127.5) as u8,
        //                 (hit.normal.y * 127.5 + 127.5) as u8,
        //                 (hit.normal.z * 127.5 + 127.5) as u8,
        //                 255,
        //             )
        //         } else {
        //             rl::Color::WHITE
        //         };
        //     }
        // }

        let mut d = rl.begin_drawing(&rt);
        // d.clear_background(rl::Color::BLACK);

        {
            let mut s = d.begin_shader_mode(&mut shader);
            s.draw_rectangle_rec(
                rl::Rectangle::new(
                    0.0,
                    0.0,
                    screen_width as f32,
                    screen_height as f32,
                ),
                rl::Color::WHITE,
            );
        }

        // for j in 0..screen_height {
        //     for i in 0..screen_width {
        //         d.draw_pixel(
        //             i as i32,
        //             j as i32,
        //             framebuffer[j * screen_width + i]
        //         );
        //     }
        // }

        d.draw_fps(5, 5);
    }
}
