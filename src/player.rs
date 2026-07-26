use crate::action::Action;
use crate::interpolation::Interpolation;
use macroquad::math::{vec2, Vec2};

pub const PLAYER_LIFE: i32 = 6;
pub const DASH_DURATION: f32 = 0.25;
pub const MIN_ATTACK_DURATION: f32 = 0.1;
pub const MAX_ATTACK_DURATION: f32 = 0.2;

pub struct Player {
    pub life: i32,
    pub pos: Vec2,
    pub looking_right: bool,
    dash: Action,
    attack: Action,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            looking_right: true,
            life: PLAYER_LIFE,
            dash: Action::new(DASH_DURATION),
            attack: Action::new(MAX_ATTACK_DURATION),
        }
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.update_attack();
        self.attack.tick(delta_s);
        self.dash.tick(delta_s);
    }
    pub fn try_dash(&mut self) {
        if self.is_alive() && !self.is_dashing() {
            self.dash.start();
        }
    }
    pub fn try_attack(&mut self) {
        if self.is_alive() {
            self.attack.start();
        }
    }
    pub fn is_alive(&self) -> bool {
        self.life > 0
    }
    pub fn is_dashing(&self) -> bool {
        self.dash.is_ongoing()
    }
    pub fn is_attacking(&self) -> bool {
        self.attack.is_ongoing()
    }
    pub fn dash(&mut self, movement: Vec2) {
        if let Some(t) = self.dash.ratio() {
            self.pos += movement
                * (Interpolation::new(1.0, 0.0).at(t))
                * (1.5 * (1.0 + self.berserk_ratio()));
        }
    }
    pub fn berserk_ratio(&self) -> f32 {
        (PLAYER_LIFE - self.life) as f32 / (PLAYER_LIFE - 1) as f32
    }
    pub fn berserk_index(&self) -> usize {
        (self.life -1).max(0) as usize
    }
    pub fn strength(&self) -> i32 {
        PLAYER_LIFE - self.life + 1
    }
    pub fn update_attack(&mut self) {
        self.attack.set_duration(
            Interpolation::new(MAX_ATTACK_DURATION, MIN_ATTACK_DURATION).at(self.berserk_ratio()),
        );
    }
}

pub fn hurt(life: &mut i32, strength: i32) {
    *life = 0.max(*life - strength);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_starting_duration() {
        let mut player = Player::new();
        assert_eq!(player.attack.duration(), MAX_ATTACK_DURATION);
        player.tick(0.016);
        assert_eq!(player.attack.duration(), MAX_ATTACK_DURATION);
    }
    #[test]
    fn test_attack_berserk_duration() {
        let mut player = Player::new();
        assert_eq!(player.attack.duration(), MAX_ATTACK_DURATION);
        hurt(&mut player.life, 1);
        player.tick(0.016);
        assert_eq!(player.attack.duration(), MIN_ATTACK_DURATION);
    }
}
