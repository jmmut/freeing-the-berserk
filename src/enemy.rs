use crate::action::Action;
use crate::player::MAX_ATTACK_DURATION;
use macroquad::math::Vec2;

pub const ENEMY_LIFE: i32 = 3;
pub const PREPARATION_S: f32 = 0.6;

pub struct Enemy {
    pub pos: Vec2,
    pub life: i32,
    pub preparing_s: f32,
    pub looking_right: bool,
    attack: Action,
    hit_player: bool,
    got_hit: bool,
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            life: ENEMY_LIFE,
            preparing_s: 0.0,
            looking_right: false,
            attack: Action::new(MAX_ATTACK_DURATION),
            hit_player: false,
            got_hit: false,
        }
    }
    pub fn tick(&mut self, delta_s: f64, in_attack_range: bool) {
        self.attack.tick(delta_s);
        if !self.attack.is_ongoing() {
            self.hit_player = false;
        }
        if self.is_alive() && in_attack_range {
            self.preparing_s += delta_s as f32;
            if self.preparing_s > PREPARATION_S {
                self.preparing_s = 0.0;
                self.attack.start();
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
        self.attack.is_ongoing()
    }
    pub fn track_hit_player(&mut self) {
        self.hit_player = true;
    }
    pub fn did_hit_player(&mut self) -> bool {
        self.hit_player
    }
    pub fn track_got_hit(&mut self) {
        self.got_hit = true;
    }
    pub fn reset_got_hit(&mut self) {
        self.got_hit = false;
    }
    pub fn did_get_hit(&self) -> bool {
        self.got_hit
    }
}
