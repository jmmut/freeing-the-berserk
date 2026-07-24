mod enemy;
mod textures;

use crate::enemy::{Enemy, ENEMY_LIFE};
use crate::textures::{Animator, Textures};
use freeing_the_berserk::AnyResult;
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

type SizeInPixels2d = Vec2;

pub const PLAYER_LIFE: i32 = 3;
pub const FONT_SIZE: f32 = 16.0;

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(e) = fallible_main().await {
        eprintln!("Error: {}", e);
    }
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

async fn fallible_main() -> AnyResult<()> {
    let map_width_meters = 8.0;
    let mut pos = vec2(0.0, 0.0);
    let mut looking_right = true;
    let mut life = PLAYER_LIFE;
    let size = vec2(1.0, 1.0);
    let attack_range = size * 0.2;
    let speed = vec2(0.03, 0.03);
    let mut enemies = vec![
        Enemy::new(vec2(-5.0, -1.0)),
        Enemy::new(vec2(-2.0, -3.0)),
        Enemy::new(vec2(3.0, 1.0)),
    ];
    let textures = Textures::load().await?;
    let mut animator = Animator::new();
    let mut previous_frame_time = now();
    loop {
        /////////// events

        let this_frame_time = now();
        let delta_s = this_frame_time - previous_frame_time;
        previous_frame_time = this_frame_time;

        let screen = vec2(screen_width(), screen_height());
        let meters_to_pixels = screen.x / map_width_meters;
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            for enemy in &mut enemies {
                enemy.life = ENEMY_LIFE;
            }
            life = PLAYER_LIFE;
        }
        let mut movement = Vec2::ZERO;
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

        let mut attacking = false;
        if is_key_pressed(KeyCode::Space) {
            attacking = true;
        }

        ///////////// logic

        animator.tick(delta_s);

        if movement != Vec2::ZERO && life > 0 {
            movement = movement.normalize() * speed.x;
            pos += movement;
            looking_right = movement.x > 0.0;
        }
        for enemy in &mut enemies {
            let enemy_to_player = pos - enemy.pos;
            let in_attack_range = is_in_attack_range(pos, enemy.pos, size, attack_range);
            enemy.tick(delta_s, in_attack_range);
            if enemy.is_alive() && enemy.is_preparing().is_none() && !in_attack_range {
                enemy.pos += enemy_to_player.normalize() * speed.x * 0.7;
            }
            if in_attack_range && enemy.is_attacking() {
                life = 0.max(life - 1);
            }
        }
        if attacking {
            for enemy in &mut enemies {
                if is_in_attack_range(pos, enemy.pos, size, attack_range) {
                    enemy.life = 0.max(enemy.life - 1);
                }
            }
        }

        ///////////// rendering

        if life <= 0 {
            text("You died. Press R to revive.", 10.0, 10.0 + FONT_SIZE);
        }
        if enemies.iter().all(|e| !e.is_alive()) {
            text(
                "You won. Press R to revive enemies.",
                10.0,
                10.0 + FONT_SIZE,
            );
        }

        for enemy in &enemies {
            let character = pos_to_rect(enemy.pos, size, screen, meters_to_pixels);
            let color = if enemy.is_alive() { WHITE } else { RED };

            // draw_rectangle(character.x, character.y, character.w, character.h, color);
            let texture = animator.choose_texture(&textures.enemies.moving);
            let params = DrawTextureParams {
                dest_size: Some(character.size()),
                ..Default::default()
            };
            draw_texture_ex(texture, character.x, character.y, color, params);
            if let Some(preparation) = enemy.is_preparing() {
                let attack = add_contour(character, attack_range * meters_to_pixels);
                let color = Color::new(0.8, 0.4, 0.4, 0.2);
                let size = attack.h * preparation;
                draw_rectangle(attack.x, attack.y + attack.h - size, attack.w, size, color);
            }
            if enemy.is_attacking() {
                let attack = add_contour(character, attack_range * meters_to_pixels);
                draw_rectangle_lines(attack.x, attack.y, attack.w, attack.h, 2.0, BLACK);
            }
        }

        let character = pos_to_rect(pos, size, screen, meters_to_pixels);
        // draw_rectangle(character.x, character.y, character.w, character.h, BLUE);
        let animation = if life > 0 && attacking {
            &textures.player.attacking
        } else if life > 0 && movement != Vec2::ZERO {
            &textures.player.moving
        } else {
            &textures.player.idle
        };
        let texture = animator.choose_texture(animation);
        let params = DrawTextureParams {
            dest_size: Some(character.size()),
            flip_x: !looking_right,
            ..Default::default()
        };
        let color = if life > 0 { WHITE } else { RED };
        draw_texture_ex(texture, character.x, character.y, color, params);

        if attacking {
            let attack = add_contour(character, attack_range * meters_to_pixels);
            draw_rectangle_lines(attack.x, attack.y, attack.w, attack.h, 2.0, BLACK);
        }

        let size_pixels = size * meters_to_pixels * 0.5;
        let pad = size_pixels * 0.5;
        for i in 0..PLAYER_LIFE {
            let x = pad.x + i as f32 * (pad.x + size_pixels.x);
            let y = screen.y - size_pixels.y - pad.y;
            let w = size_pixels.x;
            let h = size_pixels.y;
            if i < life {
                draw_rectangle(x, y, w, h, SKYBLUE);
            }
            draw_rectangle_lines(x, y, w, h, 2.0, BLACK);
        }

        next_frame().await;
    }
    Ok(())
}

fn text(text: &str, x: f32, y: f32) {
    draw_text(text, x, y, FONT_SIZE, BLACK);
}

fn is_in_attack_range(pos_1: Vec2, pos_2: Vec2, size: Vec2, attack_range: Vec2) -> bool {
    let diff = pos_2 - pos_1;
    let range = size.x + attack_range.x;
    let result = diff.length_squared() < range * range;
    result
}

fn pos_to_rect(pos: Vec2, size: Vec2, screen: Vec2, meters_to_pixels: f32) -> Rect {
    Rect::new(
        (pos.x - size.x * 0.5) * meters_to_pixels + screen.x * 0.5,
        (pos.y - size.y * 0.5) * meters_to_pixels + screen.y * 0.5,
        size.x * meters_to_pixels,
        size.y * meters_to_pixels,
    )
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
