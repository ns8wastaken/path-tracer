mod rl { pub use raylib::prelude::*; }
mod utils;
mod camera;
mod sphere;

use raylib::{prelude::{RaylibDraw, RaylibShaderModeExt}, shaders::RaylibShader};
use camera::Camera;
use sphere::Sphere;

const NEAR_CLIP_DIST: f32 = 0.1;
// const FAR_CLIP_DIST: f32 = 100.0;

macro_rules! set_shader_val {
    ($s:ident, $u:literal, $v:expr) => {
        $s.set_shader_value($s.get_shader_location($u), $v);
    };
}

fn main() {
    let (mut rl, rt) = raylib::init()
        .size(800, 600)
        .title("Hello, World")
        .build();

    gl_loader::init_gl();
    gl::load_with(|s| gl_loader::get_proc_address(s) as _);

    rl.disable_cursor();

    let screen_width = rl.get_screen_width() as usize;
    let screen_height = rl.get_screen_height() as usize;

    let mut shader = rl.load_shader(
        &rt,
        None,
        Some("src/shaders/spheretrace.frag")
    );

    let mut camera = Camera::new(
        rl::Vector3::new(-0.7891882, 1.6806532, -2.4450927),
        rl::Vector3::new(0.17874563, -0.25690365, 0.25920606),
        50.0,
    );

    set_shader_val!(shader, "u_fov", camera.fov);
    set_shader_val!(shader, "u_nearClipDist", NEAR_CLIP_DIST);
    set_shader_val!(shader, "u_screenWidth", screen_width as i32);
    set_shader_val!(shader, "u_screenHeight", screen_height as i32);

    let spheres = [
        Sphere::new(0.0, 0.0, 0.0, 1.0),
        Sphere::new(1.0, 0.0, 0.0, 1.0),
        Sphere::new(-1.0, 0.0, 0.0, 1.0),
        Sphere::new(0.0, 1.0, 0.0, 1.0),
        Sphere::new(0.0, 2.0, 0.0, 1.0),
        Sphere::new(0.0, 3.0, 0.0, 1.0),
    ];

    let mut ssbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut ssbo);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (spheres.len() * std::mem::size_of::<Sphere>()) as isize,
            spheres.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, ssbo);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    }
    set_shader_val!(shader, "u_sphereCount", spheres.len() as i32);

    let blank_texture = rl
        .load_render_texture(&rt, screen_width as u32, screen_height as u32)
        .unwrap();

    while !rl.window_should_close() {
        camera.update(&rl);

        set_shader_val!(shader, "u_cameraPos", camera.position);
        set_shader_val!(shader, "u_cameraTarget", camera.target);
        set_shader_val!(shader, "u_cameraUp", camera.get_up().normalized());

        let mut d = rl.begin_drawing(&rt);
        d.clear_background(rl::Color::BLACK);

        { // Balls :)
            let mut s = d.begin_shader_mode(&mut shader);
            s.draw_texture_pro(
                &blank_texture,
                rl::Rectangle::new(0.0, 0.0, screen_width as f32, screen_height as f32),
                rl::Rectangle::new(0.0, 0.0, screen_width as f32, screen_height as f32),
                rl::Vector2::new(0.0, 0.0),
                0.0,
                rl::Color::WHITE,
            );
        }

        d.draw_fps(5, 5);
    }
}
