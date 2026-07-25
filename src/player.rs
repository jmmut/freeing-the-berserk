use crate::interpolation::Interpolation;
use macroquad::math::{vec2, Vec2};

pub const PLAYER_LIFE: i32 = 3;
pub const DASH_DURATION: f32 = 0.25;

pub struct Player {
    pub life: i32,
    pub pos: Vec2,
    pub looking_right: bool,
    attacking: bool,
    dashing: bool,
    dash_start_s_ago: Option<f32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            looking_right: true,
            life: PLAYER_LIFE,
            attacking: false,
            dashing: false,
            dash_start_s_ago: None,
        }
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.attacking = false;
        if let Some(dash_start_s_ago) = &mut self.dash_start_s_ago {
            *dash_start_s_ago += delta_s as f32;
        }
    }
    pub fn try_dash(&mut self) {
        if self.is_alive() && !self.is_dashing() {
            self.dashing = true;
            self.dash_start_s_ago = Some(0.0);
        }
    }
    pub fn try_attack(&mut self) {
        if self.is_alive() {
            self.attacking = true;
        }
    }
    pub fn is_alive(&self) -> bool {
        self.life > 0
    }
    pub fn is_dashing(&self) -> bool {
        self.dashing
    }
    pub fn is_attacking(&self) -> bool {
        self.attacking
    }
    pub fn dash(&mut self, movement: Vec2) {
        if let Some(dash_start_s_ago) = self.dash_start_s_ago {
            if dash_start_s_ago < DASH_DURATION {
                let t = dash_start_s_ago / DASH_DURATION;
                self.pos += movement
                    * (Interpolation::new(1.0, 0.0).at(t))
                    * (1.5 * (1.0 + self.berserk_ratio()));
            } else {
                self.dash_start_s_ago = None;
                self.dashing = false;
            }
        }
    }
    pub fn berserk_ratio(&self) -> f32 {
        (PLAYER_LIFE - self.life) as f32 / (PLAYER_LIFE - 1) as f32
    }
}

pub fn hurt(life: &mut i32) {
    *life = 0.max(*life - 1);
}
