use libraytracer::types::{Vec2, Vec3};
use rand::random;

pub trait Sampler {
    fn sample(&self) -> f32;
    fn sample2(&self) -> Vec2;
    fn sample3(&self) -> Vec3;
    fn sample_hemisphere(&self) -> Vec3;
    fn sample_sphere(&self) -> Vec3;
}

pub struct RandomSampler {}
impl Sampler for RandomSampler {
    fn sample(&self) -> f32 {
        random::<f32>()
    }

    fn sample2(&self) -> Vec2 {
        Vec2::new(random::<f32>(), random::<f32>())
    }

    fn sample3(&self) -> Vec3 {
        Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
    }

    fn sample_hemisphere(&self) -> Vec3 {
        let r1 = self.sample();
        let r2 = self.sample();
        let z = (1.0 - r2).sqrt();
        let phi = std::f32::consts::TAU * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        Vec3::new(x, y, z)
    }

    fn sample_sphere(&self) -> Vec3 {
        loop {
            let p = self.sample3() * 2.0 - Vec3::new(1.0, 1.0, 1.0);
            if nalgebra_glm::length2(&p) < 1.0 {
                return p;
            }
        }
    }
}
