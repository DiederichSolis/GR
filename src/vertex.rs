use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
    pub transformed_position: Vec3,
    pub transformed_normal: Vec3,
    pub elevation: f32, // Nueva propiedad para la elevación
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2, elevation: f32) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(),
            transformed_position: position,
            transformed_normal: normal,
            elevation, // Inicializar la elevación
        }
    }

    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color,
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 0.0, 0.0),
            elevation: 0.0, // Inicializar la elevación a 0
        }
    }

    pub fn set_transformed(&mut self, position: Vec3, normal: Vec3) {
        self.transformed_position = position;
        self.transformed_normal = normal;
    }

    // Método para actualizar el color basado en la elevación
    pub fn update_color_based_on_elevation(&mut self) {
        if self.elevation < 0.0 {
            self.color = Color::new(0, 105, 148); // Color de océano
        } else if self.elevation < 0.5 {
            self.color = Color::new(34, 139, 34); // Color de tierra
        } else {
            self.color = Color::new(139, 69, 19); // Color de montaña
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: Color::black(),
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),
            elevation: 0.0, // Inicializar la elevación a 0
        }
    }
}
