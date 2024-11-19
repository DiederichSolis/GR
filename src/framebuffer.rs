use rand::Rng;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background_color: u32,
    current_color: u32,
    star_positions: Vec<(usize, usize)>, // Para mantener las posiciones de las estrellas
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
            star_positions: Vec::new(), // Inicializa el vector vacío
        }
    }

    pub fn add_stars(&mut self, star_count: usize) {
        let mut rng = rand::thread_rng();
        let star_color = 0xFFFFFF; // Color blanco para las estrellas

        for _ in 0..star_count {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            self.star_positions.push((x, y)); // Guarda la posición de la estrella
        }

        // Dibuja las estrellas en el búfer
        for &(x, y) in &self.star_positions {
            let index = y * self.width + x;
            self.buffer[index] = star_color; // Coloca las estrellas directamente en el búfer
        }
    }

    pub fn clear(&mut self) {
        // Limpia el búfer con el color de fondo
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }

        // Redibuja las estrellas después de limpiar el fondo
        let star_color = 0xFFFFFF; // Color blanco para las estrellas
        for &(x, y) in &self.star_positions {
            let index = y * self.width + x;
            self.buffer[index] = star_color; // Coloca las estrellas nuevamente
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
}