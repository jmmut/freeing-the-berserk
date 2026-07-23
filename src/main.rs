mod enemy;

use crate::enemy::{Enemy, LIFE};
use macroquad::prelude::*;

type SizeInPixels2d = Vec2;

#[macroquad::main(window_conf)]
async fn main() {
    let map_width_meters = 20.0;
    let mut pos = vec2(0.0, 0.0);
    let size = vec2(1.0, 1.0);
    let attack_range = size * 0.75;
    let speed = vec2(0.05, 0.05);
    let mut enemies = vec![
        Enemy::new(vec2(-5.0, -1.0)),
        Enemy::new(vec2(-2.0, -3.0)),
        Enemy::new(vec2(3.0, 1.0)),
    ];
    loop {
        let screen = vec2(screen_width(), screen_height());
        let meters_to_pixels = screen.x / map_width_meters;
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            for enemy in &mut enemies {
                enemy.life = LIFE;
            }
        }
        let mut movement = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            movement.y -= speed.y;
        }
        if is_key_down(KeyCode::S) {
            movement.y += speed.y;
        }
        if is_key_down(KeyCode::A) {
            movement.x -= speed.x;
        }
        if is_key_down(KeyCode::D) {
            movement.x += speed.x;
        }
        if movement != vec2(0.0, 0.0) {
            movement = movement.normalize() * speed.x;
            pos += movement;
        }

        let mut attacked = false;
        if is_key_pressed(KeyCode::Space) {
            attacked = true;
        }
        
        for enemy in &mut enemies {
            if enemy.life > 0 {
                let enemy_to_player = pos - enemy.pos;
                enemy.pos += enemy_to_player.normalize() * speed.x * 0.7;
            }
        }
        if attacked {
            for enemy in &mut enemies {
                let diff = enemy.pos - pos;
                let range = size.x + attack_range.x;
                if diff.length_squared() < range * range {
                    enemy.life = 0;
                }
            }
        }

        for enemy in &enemies {
            let character = pos_to_rect(enemy.pos, size, screen, meters_to_pixels);
            let color = if enemy.life > 0 { DARKGREEN } else { BLACK };
            draw_rectangle(character.x, character.y, character.w, character.h, color);
        }
        let character = pos_to_rect(pos, size, screen, meters_to_pixels);
        draw_rectangle(character.x, character.y, character.w, character.h, BLUE);

        if attacked {
            let attack = add_contour(character, attack_range * meters_to_pixels);
            draw_rectangle_lines(attack.x, attack.y, attack.w, attack.h, 2.0, BLACK);
        }

        next_frame().await
    }
}

fn pos_to_rect(pos: Vec2, size: Vec2, screen: Vec2, meters_to_pixels: f32) -> Rect {
    Rect::new(
        (pos.x - size.x * 0.5) * meters_to_pixels + screen.x * 0.5,
        (pos.y - size.y * 0.5) * meters_to_pixels + screen.y * 0.5,
        size.x * meters_to_pixels,
        size.y * meters_to_pixels,
    )
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Freeing the berserk".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}
pub fn add_contour(rect: Rect, size: SizeInPixels2d) -> Rect {
    let mut new_position = rect.point() - size;
    let mut new_size = rect.size() + size * 2.0;
    let center = rect.center();
    for i in 0..1 {
        if new_size[i] < 0.0 {
            // size reduced so much that the rect flips. collapse rather than invert
            new_position[i] = center[i];
            new_size[i] = 0.0;
        }
    }
    to_rect(new_position, new_size)
}
pub fn to_rect(pos: Vec2, size: Vec2) -> Rect {
    Rect::new(pos.x, pos.y, size.x, size.y)
}
