use crate::types::Vec2;

pub struct Framebuffer<P> {
    width: u32,
    height: u32,
    pixels: Vec<P>,
}

impl<P: Clone + std::ops::AddAssign> Framebuffer<P> {
    pub fn new(width: u32, height: u32, default: P) -> Self {
        Self {
            width,
            height,
            pixels: vec![default; (width * height) as usize],
        }
    }

    pub fn clear(&mut self, p: P) {
        for pixel in &mut self.pixels {
            *pixel = p.clone()
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: P) {
        let idx = y * self.width + x;
        self.pixels[idx as usize] = pixel;
    }

    pub fn accumulate_pixel(&mut self, x: u32, y: u32, pixel: P) {
        let idx = y * self.width + x;
        self.pixels[idx as usize] += pixel;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn resolution(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn enumerate_pixels<F>(&self, f: F)
    where
        F: Fn(u32, u32, &P),
    {
        (0..self.height).for_each(|y| {
            let row = y * self.width;
            (0..self.width).for_each(|x| f(x, y, &self.pixels[row as usize + x as usize]));
        });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, P> {
        self.pixels.iter()
    }
}

unsafe impl<P: Clone + std::ops::AddAssign> Send for Framebuffer<P> {}
unsafe impl<P: Clone + std::ops::AddAssign> Sync for Framebuffer<P> {}
