use nalgebra_glm::Vec3;
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;

pub struct Planet {
    pub position: Vec3,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub current_angle: f32,
    pub rotation_angle: f32,
    pub color: u32,
}

impl Planet {
    pub fn new(orbit_radius: f32, radius: f32, orbit_speed: f32, rotation_speed: f32, color: u32) -> Self {
        Planet {
            position: Vec3::new(orbit_radius, 0.0, 0.0),
            radius,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            current_angle: 0.0,
            rotation_angle: 0.0,
            color,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_angle += self.orbit_speed * delta_time;
        self.current_angle %= 2.0 * PI;
        
        self.rotation_angle += self.rotation_speed * delta_time;
        self.rotation_angle %= 2.0 * PI;
        
        self.position.x = self.orbit_radius * self.current_angle.cos();
        self.position.z = self.orbit_radius * self.current_angle.sin();
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, camera: &Camera, width: f32, height: f32) {
        let steps = 32;  // Aumentamos los pasos para un círculo más suave
        let screen_pos = camera.world_to_screen(&self.position, width, height);
        
        if screen_pos.x >= 0.0 && screen_pos.x < width &&
           screen_pos.y >= 0.0 && screen_pos.y < height {
            
            // Dibujar el planeta
            for i in 0..steps {
                let angle = (i as f32 / steps as f32) * 2.0 * PI;
                let x = screen_pos.x + (self.radius * angle.cos()) * camera.zoom_factor;
                let y = screen_pos.y + (self.radius * angle.sin()) * camera.zoom_factor;
                
                if x >= 0.0 && x < width && y >= 0.0 && y < height {
                    framebuffer.point(x as usize, y as usize, screen_pos.z);
                }
            }
        }
    }

    pub fn draw_orbit(&self, framebuffer: &mut Framebuffer, camera: &Camera, width: f32, height: f32) {
        let orbit_steps = 64;  // Más pasos para una órbita más suave
        let orbit_color = 0x444444;  // Color gris para la órbita
        
        framebuffer.set_current_color(orbit_color);
        
        // Dibujar la órbita completa
        for i in 0..orbit_steps {
            let angle = (i as f32 / orbit_steps as f32) * 2.0 * PI;
            let orbit_point = Vec3::new(
                self.orbit_radius * angle.cos(),
                0.0,
                self.orbit_radius * angle.sin()
            );
            
            let next_angle = ((i + 1) as f32 / orbit_steps as f32) * 2.0 * PI;
            let next_orbit_point = Vec3::new(
                self.orbit_radius * next_angle.cos(),
                0.0,
                self.orbit_radius * next_angle.sin()
            );
            
            let screen_pos = camera.world_to_screen(&orbit_point, width, height);
            let next_screen_pos = camera.world_to_screen(&next_orbit_point, width, height);
            
            // Dibujar línea entre puntos consecutivos
            draw_line(
                framebuffer,
                screen_pos.x as i32, screen_pos.y as i32,
                next_screen_pos.x as i32, next_screen_pos.y as i32,
                screen_pos.z
            );
        }
    }
}

// Función auxiliar para dibujar líneas (algoritmo de Bresenham)
fn draw_line(framebuffer: &mut Framebuffer, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, depth: f32) {
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    let width = framebuffer.width as i32;
    let height = framebuffer.height as i32;

    loop {
        if x1 >= 0 && x1 < width && y1 >= 0 && y1 < height {
            framebuffer.point(x1 as usize, y1 as usize, depth);
        }

        if x1 == x2 && y1 == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}

pub struct SolarSystem {
    pub sun: Planet,
    pub planets: Vec<Planet>,
}

impl SolarSystem {
    pub fn new(sun_radius: f32, sun_color: u32) -> Self {
        SolarSystem {
            sun: Planet::new(0.0, sun_radius, 0.0, 0.0, sun_color),
            planets: Vec::new(),
        }
    }

    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }

    pub fn update(&mut self, delta_time: f32) {
        for planet in &mut self.planets {
            planet.update(delta_time);
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, camera: &Camera) {
        let width = framebuffer.width as f32;
        let height = framebuffer.height as f32;

        // Primero dibujamos todas las órbitas
        for planet in &self.planets {
            planet.draw_orbit(framebuffer, camera, width, height);
        }

        // Luego dibujamos el sol y los planetas
        framebuffer.set_current_color(self.sun.color);
        self.sun.draw(framebuffer, camera, width, height);

        for planet in &self.planets {
            framebuffer.set_current_color(planet.color);
            planet.draw(framebuffer, camera, width, height);
        }
    }
}