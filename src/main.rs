use std::cmp::Ordering;
use freeing_the_berserk::enemy::{Enemy, ENEMY_LIFE};
use freeing_the_berserk::player::{hurt, Player, PLAYER_LIFE};
use freeing_the_berserk::textures::{Animator, Textures};
use freeing_the_berserk::{add_contour, pos_to_rect, AnyResult};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

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
    let mut player = Player {
        pos: vec2(0.0, 0.0),
        looking_right: true,
        life: PLAYER_LIFE,
        attacking: false,
    };
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
            player.life = PLAYER_LIFE;
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

        player.attacking = false;
        if is_key_pressed(KeyCode::Space) {
            player.attacking = true;
        }

        ///////////// logic

        animator.tick(delta_s);

        if movement != Vec2::ZERO && player.is_alive() {
            movement = movement.normalize() * speed.x;
            player.pos += movement;
            player.looking_right = movement.x > 0.0;
        }
        for i in 0..enemies.len() {
            let next_i = (i + 1) % enemies.len();
            let enemy_to_player = player.pos - enemies[i].pos;
            let in_attack_range = is_in_attack_range(player.pos, enemies[i].pos, size, attack_range);
            enemies[i].tick(delta_s, in_attack_range);
            let enemy_to_another = enemies[next_i].pos - enemies[i].pos;
            if enemies[i].is_alive() && enemies[i].is_preparing().is_none() && !in_attack_range {
                enemies[i].pos -= enemy_to_another.normalize() * speed.x * 0.1;
            }        
            if enemies[i].is_alive() && enemies[i].is_preparing().is_none() && !in_attack_range {
                enemies[i].pos += enemy_to_player.normalize() * speed.x * 0.7;
            }
            if in_attack_range && enemies[i].is_attacking() {
                hurt(&mut player.life);
            }
        }
        if player.attacking {
            for enemy in &mut enemies {
                if is_in_attack_range(player.pos, enemy.pos, size, attack_range) {
                    hurt(&mut enemy.life);
                }
            }
        }

        ///////////// rendering

        if !player.is_alive() {
            text("You died. Press R to revive.", 10.0, 10.0 + FONT_SIZE);
        }
        if enemies.iter().all(|e| !e.is_alive()) {
            text(
                "You won. Press R to revive enemies.",
                10.0,
                10.0 + FONT_SIZE,
            );
        }

        enemies.sort_by(|a, b| {
            let diff = a.pos.y - b.pos.y;
            const EPSILON: f32 = 0.1;
            if diff < -EPSILON {
                Ordering::Less
            } else if diff > EPSILON {
                Ordering::Greater
            } else {
                a.life.cmp(&b.life)
            }
        });
        for enemy in &enemies {
            if enemy.pos.y <= player.pos.y {
                draw_enemy(
                    enemy,
                    size,
                    attack_range,
                    &textures,
                    &animator,
                    screen,
                    meters_to_pixels,
                );
            }
        }

        let character = draw_player(
            &player,
            size,
            &textures,
            &animator,
            screen,
            meters_to_pixels,
            movement,
        );
        for enemy in &enemies {
            if enemy.pos.y > player.pos.y {
                draw_enemy(
                    enemy,
                    size,
                    attack_range,
                    &textures,
                    &animator,
                    screen,
                    meters_to_pixels,
                );
            }
        }

        if player.attacking {
            let attack = add_contour(character, attack_range * meters_to_pixels);
            draw_rectangle_lines(attack.x, attack.y, attack.w, attack.h, 2.0, BLACK);
        }

        draw_life(player.life, size, meters_to_pixels, screen);

        next_frame().await;
    }
    Ok(())
}

fn draw_enemy(
    enemy: &Enemy,
    size: Vec2,
    attack_range: Vec2,
    textures: &Textures,
    animator: &Animator,
    screen: Vec2,
    meters_to_pixels: f32,
) {
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

fn draw_player(
    player: &Player,
    size: Vec2,
    textures: &Textures,
    animator: &Animator,
    screen: Vec2,
    meters_to_pixels: f32,
    movement: Vec2,
) -> Rect {
    let character = pos_to_rect(player.pos, size, screen, meters_to_pixels);
    // draw_rectangle(character.x, character.y, character.w, character.h, BLUE);
    let animation = if player.is_alive() && player.attacking {
        &textures.player.attacking
    } else if player.is_alive() && movement != Vec2::ZERO {
        &textures.player.moving
    } else {
        &textures.player.idle
    };
    let texture = animator.choose_texture(animation);
    let params = DrawTextureParams {
        dest_size: Some(character.size()),
        flip_x: !player.looking_right,
        ..Default::default()
    };
    let color = if player.is_alive() { WHITE } else { RED };
    draw_texture_ex(texture, character.x, character.y, color, params);
    character
}

fn draw_life(life: i32, size: Vec2, meters_to_pixels: f32, screen: Vec2) {
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
