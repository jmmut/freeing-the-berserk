use macroquad::prelude::*;

#[macroquad::main("freeing-the-berserk")]
async fn main() {
    let map_width_meters = 20.0;
    let mut pos = vec2(0.0, 0.0);
    let mut size = vec2(1.0, 1.0);
    let mut speed = vec2(0.05, 0.05);
    loop {
        let screen = vec2(screen_width(), screen_height());
        let meters_to_pixels = screen.x / map_width_meters;
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::W) {
            pos.y -= speed.y;
        }
        if is_key_down(KeyCode::S) {
            pos.y += speed.y;
        }
        if is_key_down(KeyCode::A) {
            pos.x -= speed.x;
        }
        if is_key_down(KeyCode::D) {
            pos.x += speed.x;
        }
        draw_rectangle(
            (pos.x - size.x * 0.5) * meters_to_pixels + screen.x * 0.5,
            (pos.y - size.y * 0.5) * meters_to_pixels + screen.y * 0.5,
            size.x * meters_to_pixels,
            size.y * meters_to_pixels,
            SKYBLUE,
        );

        next_frame().await
    }
}
