use libraytracer::types::{Direction, HdrColor, Position};

#[derive(Clone, Copy, Debug)]
pub struct Payload {
    pub albedo: HdrColor,
    pub first_normal: Direction,
    pub color: HdrColor,
    pub emission: HdrColor,
    pub normal: Direction,
    pub next_direction: Direction,
    pub p: Position,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            albedo: HdrColor::new(0.0, 0.0, 0.0, 0.0),
            first_normal: Direction::new(0.0, 0.0, 0.0),
            color: HdrColor::new(0.0, 0.0, 0.0, 0.0),
            emission: HdrColor::new(0.0, 0.0, 0.0, 0.0),
            next_direction: Direction::new(0.0, 0.0, 0.0),
            normal: Direction::new(0.0, 0.0, 0.0),
            p: Position::new(0.0, 0.0, 0.0),
        }
    }
}
