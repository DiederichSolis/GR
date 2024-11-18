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
       CelestialBody::sun => sun_gradient(fragment, uniforms),
       CelestialBody::gas => gas_planet_color(fragment, uniforms),
       CelestialBody::rocky => rocky_planet_color(fragment, uniforms),
     

    }
  }

  fn earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;

    let surface_color = Color::new(255, 150, 50);
    let land_color = Color::new(255, 100, 0);
    let cloud_color = Color::new(255, 255, 255);

    let surface = uniforms.noise.get_noise_2d(position.x * 50.0, position.y * 50.0);
    let clouds = uniforms.noise.get_noise_3d(position.x * 50.0 + time, position.y * 50.0 + time * 0.5, time);

    let base_color = if surface > 0.1 {
        land_color
    } else {
        surface_color
    };

    let final_color = if clouds > 0.4 {
        base_color.lerp(&cloud_color, (clouds - 0.4) * 2.0)
    } else {
        base_color
    };

    final_color // Eliminado ajuste por intensidad de luz
}

fn moon_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 20.0, fragment.vertex_position.z * 20.0);
    let elevation = noise_value;

    let low_threshold = -0.1;
    let medium_threshold = 0.1;
    let high_threshold = 0.3;

    let dark_surface_color = Color::new(169, 169, 169);
    let light_surface_color = Color::new(211, 211, 211);
    let crater_color = Color::new(255, 255, 255);

    let color = if elevation < low_threshold {
        dark_surface_color
    } else if elevation < medium_threshold {
        light_surface_color
    } else if elevation < high_threshold {
        crater_color
    } else {
        Color::new(240, 240, 240)
    };

    color // Eliminado ajuste por intensidad de luz
}


fn sun_gradient(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Obtiene un valor de ruido para efectos adicionales (opcional).
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 10.0, fragment.vertex_position.z * 10.0);
    
    // Define colores representativos para el sol en tonos naranjas.
    let deep_orange_color = Color::new(255, 140, 0); // Naranja profundo.
    let light_orange_color = Color::new(255, 165, 80); // Naranja claro.
    let white_color = Color::new(255, 255, 255); // Blanco para el brillo.
    let warm_orange_color = Color::new(255, 200, 100); // Naranja cálido para el resplandor.

    // Determina la posición relativa del fragmento para el difuminado.
    let distance_to_sun = (fragment.vertex_position.y - 5.0).abs(); // Ajusta la altura según necesites.
    
    // Calcula un factor de difuminado basado en la distancia.
    let gradient_factor = (1.0 - distance_to_sun / 10.0).max(0.0).min(1.0);
    
    // Calcula el color difuminado combinando los colores.
    let sun_color = 
        deep_orange_color * gradient_factor * 0.5 + 
        light_orange_color * (1.0 - gradient_factor) * 0.5 + 
        warm_orange_color * gradient_factor * 0.3; // Añadiendo naranja cálido para mayor luminosidad.

    // Agrega un brillo adicional alrededor del sol.
    let glow_color = white_color * 0.3 * gradient_factor; // Brillo suave alrededor del sol.
    
    // Combina el color del sol y el brillo.
    let final_color = sun_color + glow_color;

    // Crea variaciones adicionales para simular partes del sol y su halo.
    if noise_value > 0.2 {
        let halo_color = Color::new(255, 160, 50); // Color del halo en un tono naranja más suave.
        let halo_factor = (noise_value - 0.2).min(0.5); // Intensifica el halo basado en el ruido.
        return final_color + halo_color * halo_factor; // Combina el color del halo.
    }

    final_color
}


fn gas_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Utiliza la posición del fragmento y el tiempo para generar un "seed" para el ruido.
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
    
    // Crea un generador de números aleatorios basado en el seed.
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
    
    // Genera un número aleatorio para la variación en el color.
    let random_number = rng.gen_range(0..=100);

    // Define colores base para el planeta gaseoso.
    let base_color = Color::new(70, 130, 180); // Azul
    let cloud_color = Color::new(255, 255, 255); // Blanco para nubes
    let shadow_color = Color::new(50, 50, 100); // Color oscuro para sombras

    // Calcular el factor de nubes usando el ruido
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.z * 5.0);
    let cloud_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

    // Selección de color basado en el número aleatorio para agregar variación.
    let planet_color = if random_number < 50 {
        base_color * (1.0 - cloud_factor) + cloud_color * cloud_factor
    } else {
        cloud_color * cloud_factor // Predominan las nubes
    };

    // Añadir sombras sutiles
    let shadow_factor = (1.0 - noise_value).max(0.0);
    let shadow_effect = shadow_color * shadow_factor * 0.3; // Sombra suave

    // Combina el color del planeta y las sombras
    let final_color = planet_color + shadow_effect;

    // Brillo atmosférico (opcional)
    let glow_color = Color::new(200, 200, 255); // Brillo azul claro
    let glow_factor = (1.0 - (fragment.vertex_position.y / 10.0).max(0.0).min(1.0)).max(0.0); // Basado en altura
    let final_glow = glow_color * glow_factor * 0.1; // Brillo sutil

    // Devuelve el color final combinado
    final_color + final_glow
}


fn rocky_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
     // Utiliza la posición del fragmento y el tiempo para generar un "seed" para el ruido.
let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

// Crea un generador de números aleatorios basado en el seed.
let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

// Genera un número aleatorio para la variación en el color.
let random_number = rng.gen_range(0..=100);

// Define colores base para el planeta gaseoso.
let base_color = Color::new(210, 105, 30); // Marrón rojizo (tierra)
let cloud_color = Color::new(255, 215, 0); // Dorado (nubes brillantes)
let shadow_color = Color::new(139, 69, 19); // Marrón oscuro (sombras)

// Calcular el factor de nubes usando el ruido
let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.z * 5.0);
let cloud_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

// Selección de color basado en el número aleatorio para agregar variación.
let planet_color = if random_number < 50 {
    base_color * (1.0 - cloud_factor) + cloud_color * cloud_factor
} else {
    cloud_color * cloud_factor // Predominan las nubes
};

// Añadir sombras sutiles
let shadow_factor = (1.0 - noise_value).max(0.0);
let shadow_effect = shadow_color * shadow_factor * 0.3; // Sombra suave

// Combina el color del planeta y las sombras
let final_color = planet_color + shadow_effect;

// Brillo atmosférico (opcional)
let glow_color = Color::new(255, 140, 0); // Brillo anaranjado
let glow_factor = (1.0 - (fragment.vertex_position.y / 10.0).max(0.0).min(1.0)).max(0.0); // Basado en altura
let final_glow = glow_color * glow_factor * 0.1; // Brillo sutil

// Devuelve el color final combinado
final_color + final_glow

  
}
