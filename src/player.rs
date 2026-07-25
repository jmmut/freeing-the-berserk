use macroquad::math::Vec2;

pub const PLAYER_LIFE: i32 = 3;

pub struct Player {
    pub life: i32,
    pub pos: Vec2,
    pub looking_right: bool,
    pub attacking: bool,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.life > 0
    }
}

pub fn hurt(life: &mut i32) {
    *life = 0.max(*life - 1);
}