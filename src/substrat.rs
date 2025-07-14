pub struct Substrat {
    pub dimensions: (usize, usize, usize),
    pub grid: Vec<f64>,
}

impl Substrat {
    #[must_use]
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let dimensions = (width, height, depth);
        let grid = vec![0.0; width * height * depth];
        Self { dimensions, grid }
    }

    fn _get_index(&self, x: usize, y: usize, z: usize) -> usize {
        z * self.dimensions.0 * self.dimensions.1 + y * self.dimensions.0 + x
    }
}