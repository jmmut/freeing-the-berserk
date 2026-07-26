use crate::ui::render_loading_screen;
use crate::AnyResult;
use juquad::resource_loader::resume;
use macroquad::prelude::{load_texture, Texture2D};

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
    pub async fn finish(&self) {
        while render_loading_screen(self.total, self.total).await {}
    }
}
