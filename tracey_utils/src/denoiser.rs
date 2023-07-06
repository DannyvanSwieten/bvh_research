use libraytracer::types::{Direction, HdrColor, Vec3};

#[link(name = "Denoiser")]
extern "C" {
    fn denoiser_init();
    fn denoiser_denoise(
        width: u32,
        height: u32,
        input: *const f32,
        albedo: *const f32, // optional
        normal: *const f32, // optional
        output: *mut f32,
    );
}

pub struct Denoiser {
    width: u32,
    height: u32,
}

impl Denoiser {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe {
            denoiser_init();
        }

        Self { width, height }
    }

    pub fn denoise(&self, beauty: &[f32], _albedo: &[f32], _normal: &[f32]) -> Vec<f32> {
        unsafe {
            let mut output = vec![0.0; (self.width * self.height * 3) as usize];
            denoiser_denoise(
                self.width,
                self.height,
                beauty.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                output.as_mut_ptr(),
            );
            output
        }
    }

    pub fn denoise_hdr(
        &self,
        beauty: &[HdrColor],
        albedo: &[Vec3],
        normal: &[Direction],
    ) -> Vec<f32> {
        let beauty = beauty
            .iter()
            .map(|c| Vec3::new(c.x, c.y, c.z))
            .collect::<Vec<Vec3>>();
        let mut output = vec![0.0; (self.width * self.height * 3) as usize];
        unsafe {
            denoiser_denoise(
                self.width,
                self.height,
                beauty.as_ptr() as _,
                if albedo.is_empty() {
                    std::ptr::null()
                } else {
                    albedo.as_ptr() as _
                },
                if normal.is_empty() {
                    std::ptr::null()
                } else {
                    normal.as_ptr() as _
                },
                output.as_mut_ptr(),
            );

            output
        }
    }
}
