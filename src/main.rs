use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType};

#[derive(Clone, Copy)]
pub enum CelestialBody {
    earth,
    Moon,
    sun,
    gas,
    rocky,
    nave,
    star
   
}

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
    current_body: CelestialBody,  
}

#[derive(Clone, Copy)]
pub struct Moon {
    position: Vec3,
    scale: f32,
    rotation: Vec3,
    orbit_angle: f32,
    orbit_radius: f32,
    orbit_speed: f32,
}


impl Moon {
    fn new() -> Self {
        Moon {
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.3,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            orbit_angle: 0.0,
            orbit_radius: 2.0,
            orbit_speed: 0.02,
        }
      
    }

    fn update(&mut self) {
        self.orbit_angle += self.orbit_speed;
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
        self.rotation.y += 0.01;
    }
}

    #[derive(Clone, Copy)]
    pub struct Sun {
        position: Vec3,
        scale: f32,
        rotation: Vec3,
        orbit_angle: f32,
        orbit_radius: f32,
        orbit_speed: f32,
    }

    impl Sun {
        fn new() -> Self {
            Sun {
                position: Vec3::new(0.0, 0.0, 0.0),
                scale: 0.5, // Puedes ajustar el tamaño según prefieras
                rotation: Vec3::new(0.0, 0.0, 0.0),
                orbit_angle: 0.0,
                orbit_radius: 3.0, // Ajusta el radio de la órbita
                orbit_speed: 0.015, // Ajusta la velocidad de la órbita
            }
        }

        fn update(&mut self) {
            self.orbit_angle += self.orbit_speed;
            self.position.x = self.orbit_angle.cos() * self.orbit_radius;
            self.position.z = self.orbit_angle.sin() * self.orbit_radius;
            self.rotation.y += 0.01; // Ajusta la rotación si lo deseas
        }
    }


    #[derive(Clone, Copy)]
pub struct GasPlanet {
    position: Vec3,
    scale: f32,
    rotation: Vec3,
    orbit_angle: f32,
    orbit_radius: f32,
    orbit_speed: f32,
}

impl GasPlanet {
    fn new() -> Self {
        GasPlanet {
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.6, // Tamaño del planeta de gas
            rotation: Vec3::new(0.0, 0.0, 0.0),
            orbit_angle: 0.0,
            orbit_radius: 4.0, // Radio de la órbita
            orbit_speed: 0.008, // Velocidad de la órbita
        }
    }

    fn update(&mut self) {
        self.orbit_angle += self.orbit_speed;
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
        self.rotation.y += 0.01; // Rotación ajustable
    }

    
}

    #[derive(Clone, Copy)]
    pub struct Star {
        position: Vec3,
        scale: f32,
        rotation: Vec3,
        orbit_angle: f32,
        orbit_radius: f32,
        orbit_speed: f32,
    }

    impl Star {
        fn new() -> Self {
            Star {
                position: Vec3::new(0.0, 0.0, 0.0),
                scale: 0.7, // Tamaño de Star
                rotation: Vec3::new(0.0, 0.0, 0.0),
                orbit_angle: 0.0,
                orbit_radius: 5.0, // Radio de la órbita de Star
                orbit_speed: 0.006, // Velocidad de la órbita de Star
            }
        }

        fn update(&mut self) {
            self.orbit_angle += self.orbit_speed;
            self.position.x = self.orbit_angle.cos() * self.orbit_radius;
            self.position.z = self.orbit_angle.sin() * self.orbit_radius;
            self.rotation.y += 0.01; // Rotación ajustable
        }
}

    #[derive(Clone, Copy)]
    pub struct RockyPlanet {
        position: Vec3,
        scale: f32,
        rotation: Vec3,
        orbit_angle: f32,
        orbit_radius: f32,
        orbit_speed: f32,
    }

    impl RockyPlanet {
        fn new() -> Self {
            RockyPlanet {
                position: Vec3::new(0.0, 0.0, 0.0),
                scale: 0.85, 
                rotation: Vec3::new(0.0, 0.0, 0.0),
                orbit_angle: 0.0,
                orbit_radius: 6.0, // Radio de la órbita
                orbit_speed: 0.004, // Velocidad de la órbita
            }
        }

        fn update(&mut self) {
            self.orbit_angle += self.orbit_speed;
            self.position.x = self.orbit_angle.cos() * self.orbit_radius;
            self.position.z = self.orbit_angle.sin() * self.orbit_radius;
            self.rotation.y += 0.01; 
        }
    }


    #[derive(Clone, Copy)]
    pub struct Startplanet {
        position: Vec3,
        scale: f32,
        rotation: Vec3,
        orbit_angle: f32,
        orbit_radius: f32,
        orbit_speed: f32,
    }

    impl Startplanet {
        fn new() -> Self {
            Startplanet {
                position: Vec3::new(0.0, 0.0, 0.0),
                scale: 0.85, 
                rotation: Vec3::new(0.0, 0.0, 0.0),
                orbit_angle: 0.0,
                orbit_radius: 8.0, // Radio de la órbita
                orbit_speed: 0.004, // Velocidad de la órbita
            }
        }

        fn update(&mut self) {
            self.orbit_angle += self.orbit_speed;
            self.position.x = self.orbit_angle.cos() * self.orbit_radius;
            self.position.z = self.orbit_angle.sin() * self.orbit_radius;
            self.rotation.y += 0.01; 
        }
    }

    #[derive(Clone, Copy)]
pub struct Spaceship {
    position: Vec3,
    scale: f32,
    rotation: Vec3,
    orbit_angle: f32,
    orbit_radius: f32,
    orbit_speed: f32,
}

impl Spaceship {
    fn new() -> Self {
        Spaceship {
            position: Vec3::new(2.0, 0.0, -5.0),
            scale: 1.0,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            orbit_angle: 0.0,
            orbit_radius: 9.0, // Radio de la órbita de la nave
            orbit_speed: 0.01, // Velocidad de la órbita de la nave
        }
    }

    fn update(&mut self) {
        // Actualizar el ángulo de la órbita
        self.orbit_angle += self.orbit_speed;
        // Calcular la nueva posición de la nave en la órbita
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
        // Actualizar la rotación para simular el giro de la nave
        self.rotation.y += 0.01;
    }
}

fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 10.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}


fn main() {
    let window_width = 900;
    let window_height = 900;
    let framebuffer_width = 760;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "sitema solar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);
    framebuffer.add_stars(10000);  // Agregar 100 estrellas al espacio

    let translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 1.0f32;

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let obj: Obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let spaceship_obj = Obj::load("assets/models/nave.obj").expect("Failed to load spaceship.obj");
    println!("Vertices cargados: {}", spaceship_obj.get_vertex_array().len());
    let vertex_arrays = obj.get_vertex_array();
    let vertex_array_nave = spaceship_obj.get_vertex_array();

    
    let mut time = 0;
    let mut current_body = CelestialBody::earth;
    let mut moon = Moon::new();
    let mut sun = Sun::new();
    let mut gas_planet = GasPlanet::new();
    let mut star = Star::new();
    let mut rocky_planet = RockyPlanet::new();
    let mut Startplanet = Startplanet::new();
    let mut nave = Spaceship::new();





    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;
        rotation.y += 0.01;

        handle_input(&window, &mut camera);

        framebuffer.clear();

        let noise = create_noise();
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
        
        let uniforms = Uniforms { 
            model_matrix, 
            view_matrix, 
            projection_matrix, 
            viewport_matrix,
            time,
            noise,
            current_body,
        };

        render(&mut framebuffer, &uniforms, &vertex_arrays);

        // Renderizar la luna 
        if let CelestialBody::earth = current_body {
            moon.update();
            
            let moon_model_matrix = create_model_matrix(
                moon.position,
                moon.scale,
                moon.rotation
            );

            let moon_uniforms = Uniforms {
                model_matrix: moon_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: CelestialBody::Moon,
            };

            render(&mut framebuffer, &moon_uniforms, &vertex_arrays);
            sun.update();
    
            let sun_model_matrix = create_model_matrix(
                sun.position,
                sun.scale,
                sun.rotation
            );
        
            let sun_uniforms = Uniforms {
                model_matrix: sun_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: CelestialBody::sun, // Asegúrate de que el sol tenga su propio tipo
            };
        
            render(&mut framebuffer, &sun_uniforms, &vertex_arrays);

                        // En el loop principal
            gas_planet.update();

            let gas_planet_model_matrix = create_model_matrix(
                gas_planet.position,
                gas_planet.scale,
                gas_planet.rotation
            );

            let gas_planet_uniforms = Uniforms {
                model_matrix: gas_planet_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: CelestialBody::gas, // Tipo de cuerpo para el planeta de gas
            };

            // Renderizar el planeta de gas
            render(&mut framebuffer, &gas_planet_uniforms, &vertex_arrays);


            rocky_planet.update(); // Actualizar el planeta rocoso

            // Crear matriz de modelo para cada cuerpo celeste y renderizar
            let rocky_model_matrix = create_model_matrix(rocky_planet.position, rocky_planet.scale, rocky_planet.rotation);
            let rocky_uniforms = Uniforms {
                model_matrix: rocky_model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                    noise: create_noise(),
                current_body: CelestialBody::rocky, // Marcar como el planeta actual
            };
    
            render(&mut framebuffer, &rocky_uniforms, &vertex_arrays);
    

            // Actualizar Star
            star.update();

            let star_model_matrix = create_model_matrix(
                star.position,
                star.scale,
                star.rotation
            );

            let star_uniforms = Uniforms {
                model_matrix: star_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: CelestialBody::star, // Enum puede cambiarse a algo representativo de Star
            };

            render(&mut framebuffer, &star_uniforms, &vertex_arrays);


            Startplanet.update(); // Actualizar el planeta rocoso

            // Crear matriz de modelo para cada cuerpo celeste y renderizar
            let Startplanet_matrix = create_model_matrix(Startplanet.position, Startplanet.scale, Startplanet.rotation);
            let staruniforms = Uniforms {
                model_matrix: Startplanet_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                    noise: create_noise(),
                current_body: CelestialBody::star, // Marcar como el planeta actual
            };
    
            render(&mut framebuffer, &staruniforms, &vertex_arrays);

               
            nave.update();
                // Crear matriz de modelo para la nave
            let spacecraft_model_matrix = create_model_matrix(nave.position, 0.09, nave.rotation);
            let spacecraft_uniforms = Uniforms {
                model_matrix: spacecraft_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: CelestialBody::nave, // Marcar como la nave actual
            };

            // Renderizar la nave
            render(&mut framebuffer, &spacecraft_uniforms, &vertex_array_nave);
                


                    }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}