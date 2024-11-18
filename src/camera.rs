use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::f32::consts::PI;

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub zoom_factor: f32,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        let front = (target - position).normalize();
        let right = front.cross(&up).normalize();
        let up = right.cross(&front).normalize();
        
        // Calcular yaw y pitch iniciales
        let yaw = front.z.atan2(front.x);
        let pitch = (-front.y).asin();

        Camera {
            position,
            front,
            up,
            right,
            yaw,
            pitch,
            zoom_factor: 1.0,
            has_changed: true,
        }
    }

    fn update_camera_vectors(&mut self) {
        self.front = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();

        self.right = self.front.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(&self.front).normalize();
        self.has_changed = true;
    }

    // Simplificamos la conversión de mundo a pantalla para mantener la visualización 2D
    pub fn world_to_screen(&self, world_pos: &Vec3, width: f32, height: f32) -> Vec3 {
        // Calculamos la posición relativa a la cámara
        let relative_pos = world_pos - self.position;
        
        // Proyectamos el punto en el plano de la cámara
        let forward = self.front.normalize();
        let right = self.right.normalize();
        let up = self.up.normalize();
        
        // Calculamos las coordenadas en el espacio de la cámara
        let right_proj = relative_pos.dot(&right);
        let up_proj = relative_pos.dot(&up);
        let forward_proj = relative_pos.dot(&forward);
        
        // Solo proyectamos si el punto está frente a la cámara
        if forward_proj > 0.0 {
            // Aplicamos una perspectiva simple
            let scale = self.zoom_factor / forward_proj;
            
            // Convertimos a coordenadas de pantalla
            let screen_x = width / 2.0 + right_proj * scale * width / 2.0;
            let screen_y = height / 2.0 - up_proj * scale * height / 2.0;
            
            Vec3::new(screen_x, screen_y, forward_proj)
        } else {
            // Si el punto está detrás de la cámara, lo colocamos fuera de la pantalla
            Vec3::new(-1.0, -1.0, forward_proj)
        }
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.position += self.front * delta;
        self.has_changed = true;
    }

    pub fn move_right(&mut self, delta: f32) {
        self.position += self.right * delta;
        self.has_changed = true;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.position += Vec3::new(0.0, delta, 0.0);
        self.has_changed = true;
    }

    pub fn rotate(&mut self, yaw_delta: f32, pitch_delta: f32) {
        self.yaw += yaw_delta;
        self.pitch += pitch_delta;

        // Limitar el pitch
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        self.update_camera_vectors();
    }

    pub fn zoom(&mut self, factor: f32) {
        self.zoom_factor = (self.zoom_factor * factor).clamp(0.1, 10.0);
        self.has_changed = true;
    }

    pub fn check_if_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            true
        } else {
            false
        }
    }
}