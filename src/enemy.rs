use macroquad::math::Vec2;

const LIFE: i32 = 5;

pub struct Enemy {
    pub pos: Vec2,
    pub life: i32,
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, life: LIFE }
    }
}
