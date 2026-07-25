use macroquad::math::{Rect, Vec2};
use std::error::Error;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;
pub type SizeInPixels2d = Vec2;

pub fn pos_to_rect(pos: Vec2, size: Vec2, screen: Vec2, meters_to_pixels: f32) -> Rect {
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
