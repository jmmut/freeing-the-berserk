use crate::{add_contour, to_rect, AnyResult};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::resource_loader::resume;
use macroquad::color::{Color, BLACK};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{
    clear_background, load_texture, next_frame, screen_height, screen_width, Texture2D,
};

pub struct Loader {
    pub done: i32,
    pub total: i32,
}

impl Loader {
    pub async fn new(resources_estimate_count: i32) -> Self {
        render_loading_screen(0, resources_estimate_count).await;
        Self {
            done: 0,
            total: resources_estimate_count,
        }
    }
    pub async fn load_texture(&mut self, path: String) -> AnyResult<Texture2D> {
        let mut pin_future = Box::pin(load_texture(&path));
        loop {
            render_loading_screen(self.done, self.total).await;
            match resume(pin_future.as_mut()) {
                None => {
                    // it's still loading, render another frame instead of blocking
                }
                Some(loaded) => {
                    self.done += 1;
                    return Ok(loaded?);
                }
            }
        }
    }
}

pub async fn render_loading_screen(done: i32, total: i32) {
    let screen = vec2(screen_width(), screen_height());
    let rect = add_contour(to_rect(Vec2::ZERO, screen), -screen * vec2(0.2, 0.49));
    let mut rect_progress = rect;
    rect_progress.w = rect_progress.w * done as f32 / total as f32;
    clear_background(BLACK);
    const ARMOR_GREY: Color = Color::from_rgba(120, 120, 120, 255);
    const HAIR_RED: Color = Color::from_rgba(109, 0, 22, 255);
    draw_rect(rect_progress, HAIR_RED);
    draw_rect_lines(rect, 2.0, ARMOR_GREY);
    next_frame().await;
}
