use crate::{add_contour, to_rect, AnyResult};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::resource_loader::resume;
use macroquad::color::{Color, BLACK};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{
    clear_background, load_texture, next_frame, screen_height, screen_width, Texture2D,
};
use crate::ui::render_loading_screen;

pub struct Loader {
    pub done: i32,
    pub total: i32,
}

impl Loader {
    pub async fn new(resources_estimate_count: i32) -> Self {
        render_loading_screen(0, resources_estimate_count).await; // TODO: take Box<Fn>
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
