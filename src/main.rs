use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::{Instant, Duration};

mod framebuffer;
mod camera;
mod solar_system;

use framebuffer::Framebuffer;
use camera::Camera;
use solar_system::{SolarSystem, Planet};

fn main() {
    let width = 800;
    let height = 600;
    let mut window = Window::new(
        "Sistema Solar - Gráficas por Computadora",
        width,
        height,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    let mut framebuffer = Framebuffer::new(width, height);
    
    let mut camera = Camera::new(
        Vec3::new(0.0, 150.0, 400.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut solar_system = SolarSystem::new(30.0, 0xFFFF00);
    
    solar_system.add_planet(Planet::new(150.0, 15.0, 0.8, 2.0, 0x00FF00));
    solar_system.add_planet(Planet::new(250.0, 20.0, 0.6, 1.5, 0x0000FF));
    solar_system.add_planet(Planet::new(350.0, 18.0, 0.4, 1.8, 0xFF0000));
    solar_system.add_planet(Planet::new(450.0, 12.0, 0.3, 1.8, 0xFF00FF));

    let mut previous_time = Instant::now();
    let mut mouse_pos = (0.0f32, 0.0f32);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(previous_time).as_secs_f32();
        previous_time = current_time;

        let camera_speed = 100.0 * delta_time;
        let rotation_speed = 1.5 * delta_time;
        
        if window.is_key_down(Key::W) {
            camera.move_forward(camera_speed);
        }
        if window.is_key_down(Key::S) {
            camera.move_forward(-camera_speed);
        }
        
        if window.is_key_down(Key::A) {
            camera.move_right(-camera_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(camera_speed);
        }
        
        if window.is_key_down(Key::Space) {
            camera.move_up(camera_speed);
        }
        if window.is_key_down(Key::LeftShift) {
            camera.move_up(-camera_speed);
        }

        if window.is_key_down(Key::Left) {
            camera.rotate(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.rotate(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.rotate(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.rotate(0.0, rotation_speed);
        }
        
        if window.is_key_down(Key::Q) {
            camera.zoom(1.0 + delta_time);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(1.0 - delta_time);
        }

        framebuffer.clear();
        solar_system.update(delta_time);
        solar_system.draw(&mut framebuffer, &camera);

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .expect("No se pudo actualizar la ventana");
    }
}