use freeing_the_berserk::enemy::{Enemy, ENEMY_LIFE};
use freeing_the_berserk::player::{hurt, Player, PLAYER_LIFE};
use freeing_the_berserk::textures::{Animator, Textures};
use freeing_the_berserk::{add_contour, pixel_to_pos, pos_to_rect, AnyResult};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use std::cmp::Ordering;
use freeing_the_berserk::loader::Loader;

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
    let mut loader = Loader::new();
    let map_width_meters = 10.0;
    let mut player = Player::new();
    let size = vec2(1.0, 1.0);
    let border_meters = vec2(1.2, 0.7);
    let attack_range = size * 0.2;
    let speed = vec2(0.03, 0.03);
    let mut enemies = generate_enemies();
    let textures = Textures::load(&mut loader).await?;
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
            enemies = generate_enemies();
            player.life = PLAYER_LIFE;
        }
        let mut movement = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            movement.y -= speed.y;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            movement.y += speed.y;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            movement.x -= speed.x;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            movement.x += speed.x;
        }

        ///////////// logic

        player.tick(delta_s);

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::J) {
            player.try_attack();
        }
        if is_key_pressed(KeyCode::K)
            || is_key_pressed(KeyCode::LeftShift)
            || is_key_pressed(KeyCode::RightShift)
        {
            player.try_dash();
        }

        animator.tick(delta_s);

        if movement != Vec2::ZERO && player.is_alive() && !player.is_attacking() {
            movement = movement.normalize() * speed.x;
            player.pos += movement;
            maybe_flip(speed, movement, &mut player.looking_right);
            if player.is_dashing() {
                player.dash(movement);
            }
        }
        let top_left = pixel_to_pos(border_meters * meters_to_pixels, screen, meters_to_pixels);
        let bottom_right = pixel_to_pos(
            screen - border_meters * meters_to_pixels,
            screen,
            meters_to_pixels,
        );
        player.pos = player.pos.clamp(top_left, bottom_right);

        for i in 0..enemies.len() {
            let next_i = (i + 1) % enemies.len();
            let enemy_to_player = player.pos - enemies[i].pos;
            let in_attack_range =
                is_in_attack_range(player.pos, enemies[i].pos, size, attack_range);
            enemies[i].tick(delta_s, in_attack_range);
            let enemy_to_another = enemies[next_i].pos - enemies[i].pos;
            let mut movement = Vec2::ZERO;
            if enemies[i].is_alive() && enemies[i].is_preparing().is_none() && !in_attack_range {
                movement -= enemy_to_another.normalize() * speed.x * 0.3;
            }
            if enemies[i].is_alive() && enemies[i].is_preparing().is_none() && !in_attack_range {
                movement += enemy_to_player.normalize() * speed.x * 0.7;
            }

            enemies[i].pos += movement;
            maybe_flip(speed, movement, &mut enemies[i].looking_right);
            if in_attack_range && enemies[i].is_attacking() && !enemies[i].did_hit_player() {
                hurt(&mut player.life, 1);
                enemies[i].track_hit_player();
            }
        }

        if player.is_attacking() {
            for enemy in &mut enemies {
                if is_in_attack_range(player.pos, enemy.pos, size, attack_range)
                    && !enemy.did_get_hit()
                {
                    enemy.track_got_hit();
                    hurt(&mut enemy.life, player.strength());
                }
            }
        } else {
            for enemy in &mut enemies {
                enemy.reset_got_hit();
            }
        }

        ///////////// rendering
        let params = DrawTextureParams {
            dest_size: Some(screen),
            ..Default::default()
        };
        draw_texture_ex(&textures.background[0], 0.0, 0.0, WHITE, params);

        // let top_left = border_meters * 0.5 * meters_to_pixels;
        // let arena_size = screen - 2.0 * top_left;
        // let arena_color = Color::new(1.00, 0.63, 0.00, 0.3);
        // draw_rectangle_lines(
        //     top_left.x,
        //     top_left.y,
        //     arena_size.x,
        //     arena_size.y,
        //     10.0,
        //     arena_color,
        // );

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

        let _character = draw_player(
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
        //
        // if player.is_attacking() {
        //     let attack = add_contour(character, attack_range * meters_to_pixels);
        //     draw_rectangle_lines(attack.x, attack.y, attack.w, attack.h, 10.0, BLACK);
        // }
        // if player.is_dashing() {
        //     draw_rectangle_lines(
        //         character.x,
        //         character.y,
        //         character.w,
        //         character.h,
        //         20.0,
        //         BLUE,
        //     );
        // }

        // draw_life(player.life, size, meters_to_pixels, screen);

        let text_x = screen.x * 0.5;
        let text_y = screen.y - 2.0 * FONT_SIZE;
        if !player.is_alive() {
            text("You died. Press R to revive.", text_x, text_y);
        }
        if enemies.iter().all(|e| !e.is_alive()) {
            text("You won. Press R to revive enemies.", text_x, text_y);
        }

        next_frame().await;
    }
    Ok(())
}

fn generate_enemies() -> Vec<Enemy> {
    vec![
        Enemy::new(vec2(-5.0, -1.0)),
        Enemy::new(vec2(-2.0, -3.0)),
        Enemy::new(vec2(3.0, 1.0)),
        Enemy::new(vec2(10.0, -3.0)),
        Enemy::new(vec2(19.0, 3.0)),
        Enemy::new(vec2(15.0, -3.0)),
    ]
}

fn maybe_flip(speed: Vec2, movement: Vec2, looking_right: &mut bool) {
    if *looking_right {
        if movement.x < -speed.x * 0.15 {
            *looking_right = false;
        }
    } else {
        if movement.x > speed.x * 0.15 {
            *looking_right = true;
        }
    }
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
    let mut character_texture = character;
    character_texture.w = textures.character_size.x * meters_to_pixels;
    character_texture.x += character.w * 0.5 - character_texture.w * 0.5;
    let color = if enemy.is_alive() { WHITE } else { RED };

    // draw_rectangle(character.x, character.y, character.w, character.h, color);

    let animation = if enemy.is_alive() && enemy.is_attacking() {
        &textures.enemies.attack
    } else if enemy.is_alive() {
        &textures.enemies.walk
    } else {
        &textures.enemies.idle
    };
    let texture = animator.choose_texture(animation);
    let params = DrawTextureParams {
        dest_size: Some(character_texture.size()),
        flip_x: !enemy.looking_right,
        ..Default::default()
    };
    draw_texture_ex(
        texture,
        character_texture.x,
        character_texture.y,
        color,
        params,
    );
    if let Some(preparation) = enemy.is_preparing() {
        let attack = add_contour(character, attack_range * meters_to_pixels);
        let color = Color::new(0.8, 0.4, 0.4, 0.2);
        let size = attack.h * preparation;
        draw_rectangle(attack.x, attack.y + attack.h - size, attack.w, size, color);
    }
    if enemy.is_attacking() {
        // let attack = add_contour(character, attack_range * meters_to_pixels);
        // draw_rectangle_lines(attack.x, attack.y, attack.w, attack.h, 10.0, BLACK);
    }
    let size_pixels = size * meters_to_pixels * 0.1;
    // let pad = size_pixels * 0.5;
    let pad = Vec2::ZERO;
    for i in 0..ENEMY_LIFE {
        let x = character.x + character.w * 0.5 - ENEMY_LIFE as f32 * 0.5 * (pad.x + size_pixels.x)
            + pad.x
            + i as f32 * (pad.x + size_pixels.x);
        let y = character.y - pad.y * 4.0;
        let w = size_pixels.x;
        let h = size_pixels.y;
        if i < enemy.life {
            draw_rectangle(x, y, w, h, RED);
        }
        draw_rectangle_lines(x, y, w, h, 2.0, BLACK);
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
    let mut character_texture = character;
    character_texture.w = textures.character_size.x * meters_to_pixels;
    character_texture.x += character.w * 0.5 - character_texture.w * 0.5;
    // draw_rectangle(character.x, character.y, character.w, character.h, BLUE);
    let animations = &textures.player[player.berserk_index()];
    let animation = if player.is_alive() && player.is_attacking() {
        &animations.attack
    } else if player.is_alive() && player.is_dashing() {
        &animations.dash
    } else if player.is_alive() && movement != Vec2::ZERO {
        &animations.walk
    } else {
        &animations.idle
    };
    let texture = animator.choose_texture(animation);
    let params = DrawTextureParams {
        dest_size: Some(character_texture.size()),
        flip_x: !player.looking_right,
        ..Default::default()
    };
    let color = if player.is_alive() { WHITE } else { RED };
    draw_texture_ex(
        texture,
        character_texture.x,
        character_texture.y,
        color,
        params,
    );
    let size_pixels = size * meters_to_pixels * 0.1;
    let pad = size_pixels * 0.5;
    for i in 0..PLAYER_LIFE {
        let x = character.x + character.w * 0.5
            - PLAYER_LIFE as f32 * 0.5 * (pad.x + size_pixels.x)
            + pad.x
            + i as f32 * (pad.x + size_pixels.x);
        let y = character.y - pad.y * 4.0;
        let w = size_pixels.x;
        let h = size_pixels.y;
        if i < player.life {
            draw_rectangle(x, y, w, h, BLUE);
        }
        draw_rectangle_lines(x, y, w, h, 2.0, BLACK);
    }
    character
}

#[allow(unused)]
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
