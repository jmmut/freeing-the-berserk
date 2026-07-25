use macroquad::math::Vec2;

pub const ENEMY_LIFE: i32 = 3;
pub const PREPARATION_S: f32 = 1.0;

pub struct Enemy {
    pub pos: Vec2,
    pub life: i32,
    pub preparing_s: f32,
    pub attacking: bool,
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            life: ENEMY_LIFE,
            preparing_s: 0.0,
            attacking: false,
        }
    }
    pub fn tick(&mut self, delta_s: f64, in_attack_range: bool) {
        self.attacking = false;
        if self.is_alive() && in_attack_range {
            self.preparing_s += delta_s as f32;
            if self.preparing_s > PREPARATION_S {
                self.preparing_s = 0.0;
                self.attacking = true;
            }
        } else {
            self.preparing_s = 0.0;
        }
    }
    pub fn is_alive(&self) -> bool {
        self.life > 0
    }
    pub fn is_preparing(&self) -> Option<f32> {
        if self.preparing_s > 0.0 {
            Some(self.preparing_s / PREPARATION_S)
        } else {
            None
        }
    }
    pub fn is_attacking(&self) -> bool {
        self.attacking
    }
}
