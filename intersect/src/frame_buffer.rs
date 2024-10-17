pub struct Framebuffer<P> {
    width: usize,
    height: usize,
    pixels: Vec<P>,
}

impl<P: Clone + std::ops::AddAssign> Framebuffer<P> {
    pub fn new(width: usize, height: usize, default: P) -> Self {
        Self {
            width,
            height,
            pixels: vec![default; width * height],
        }
    }

    pub fn clear(&mut self, p: P) {
        for pixel in &mut self.pixels {
            *pixel = p.clone()
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: P) {
        let idx = y * self.width + x;
        self.pixels[idx] = pixel;
    }

    pub fn accumulate_pixel(&mut self, x: usize, y: usize, pixel: P) {
        let idx = y * self.width + x;
        self.pixels[idx] += pixel;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn enumerate_pixels<F>(&self, f: F)
    where
        F: Fn(usize, usize, &P),
    {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| f(x, y, &self.pixels[y * self.width + x]));
        });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, P> {
        self.pixels.iter()
    }

    pub fn pixels(&self) -> &[P] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [P] {
        &mut self.pixels
    }
}

unsafe impl<P: Clone + std::ops::AddAssign> Send for Framebuffer<P> {}
unsafe impl<P: Clone + std::ops::AddAssign> Sync for Framebuffer<P> {}
