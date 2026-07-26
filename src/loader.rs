use macroquad::prelude::{load_texture, Texture2D};
use crate::AnyResult;

pub struct Loader {
    
}

impl Loader {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn load_texture(&mut self, path: String) -> AnyResult<Texture2D>{
        Ok(load_texture(&path).await?)
    }
}
