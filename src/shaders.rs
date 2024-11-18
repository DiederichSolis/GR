use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng; // Mantener solo si se usa
use rand::SeedableRng; // Mantener solo si se usa
use rand::rngs::StdRng; // Mantener solo si se usa
use crate::CelestialBody;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  let position = Vec4::new(
      vertex.position.x,
      vertex.position.y,
      vertex.position.z,
      1.0
  );

  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  let w = transformed.w;
  let transformed_position = Vec4::new(
      transformed.x / w,
      transformed.y / w,
      transformed.z / w,
      1.0
  );

  let screen_position = uniforms.viewport_matrix * transformed_position; // Asegúrate de que esta línea esté presente

  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Actualiza el color basado en la elevación
  let mut new_vertex = Vertex {
      position: vertex.position,
      normal: vertex.normal,
      tex_coords: vertex.tex_coords,
      color: vertex.color, // Color inicial
      transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
      transformed_normal: transformed_normal,
      elevation: vertex.elevation, // Mantiene la elevación original
  };

  // Actualiza el color del vértice basado en la elevación
  new_vertex.update_color_based_on_elevation();

  new_vertex
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    match uniforms.current_body {
       CelestialBody::earth => earth(fragment, uniforms),
       CelestialBody::Moon => moon_color(fragment, uniforms),
        
    }
  }

  fn earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;
  
    // Colores modificados a tonos naranjas
    let surface_color = Color::new(255, 150, 50);  // Naranja para los océanos
    let land_color = Color::new(255, 100, 0);      // Naranja oscuro para la tierra
    let cloud_color = Color::new(255, 255, 255);   // Blanco para las nubes
    
    // Cálculos de ruido para la superficie
    let surface = uniforms.noise.get_noise_2d(
        position.x * 50.0,  // Reducir la escala del ruido para ampliar el océano
        position.y * 50.0   // Reducir la escala del ruido para ampliar el océano
    );
    
    // Ruido para las nubes, con más capas de movimiento
    let clouds_1 = uniforms.noise.get_noise_3d(
        position.x * 50.0 + time,
        position.y * 50.0 + time * 0.5,
        time
    );
    
    let clouds_2 = uniforms.noise.get_noise_3d(
        position.x * 60.0 + time * 1.5,
        position.y * 60.0 + time * 0.8,
        time * 0.5
    );
    
    // Mezcla de las dos capas de nubes para dar más variabilidad
    let clouds = (clouds_1 + clouds_2) * 0.5;  // Promedio de las dos capas para más nubes

    // Determinar el color base: ahora el umbral para océano es más bajo, más superficie es océano
    let base_color = if surface > 0.1 {  // Reducir el umbral para que más área sea tierra
        land_color  // Si es tierra, usar color naranja oscuro
    } else {
        surface_color  // Si es agua, usar color naranja claro
    };
    
    // Mezcla con las nubes
    let final_color = if clouds > 0.4 {  // Ajusta este valor para mayor o menor densidad de nubes
        base_color.lerp(&cloud_color, (clouds - 0.4) * 2.0)
    } else {
        base_color
    };
    
    final_color * fragment.intensity  // Ajustar el color según la intensidad de luz
}




fn moon_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Aumentar la escala del ruido para más detalles
  let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 20.0, fragment.vertex_position.z * 20.0);
  
  let elevation = noise_value; // Puedes combinar varios niveles de ruido si lo deseas

  // Umbrales
  let low_threshold = -0.1; 
  let medium_threshold = 0.1;
  let high_threshold = 0.3; // Agregar un nuevo umbral para cráteres

  // Colores representativos
  let dark_surface_color = Color::new(169, 169, 169); // Gris oscuro
  let light_surface_color = Color::new(211, 211, 211); // Gris claro
  let crater_color = Color::new(255, 255, 255);       // Blanco para los cráteres

  // Determinamos el color
  let color = if elevation < low_threshold {
      dark_surface_color
  } else if elevation < medium_threshold {
      light_surface_color
  } else if elevation < high_threshold {
      crater_color // Área de cráteres
  } else {
      Color::new(240, 240, 240) // Color para áreas muy altas
  };

  color * fragment.intensity
}
