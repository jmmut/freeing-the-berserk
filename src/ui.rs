use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::elm::button::Button;
use juquad::elm::container::Container;
use juquad::elm::style::Style;
use juquad::elm::text::Text;
use juquad::elm::widget::{compute_layout, RenderableWidget};
use juquad::lazy::Pad;
use juquad::widgets::anchor::{Horizontal, Layout, Vertical};
use juquad::widgets::{Coloring, StateColor};
use macroquad::color::{Color, BLACK};
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width};
use crate::{add_contour, to_rect};

pub const FONT_SIZE: f32 = 16.0;

const BELT_LIGHTRED: Color = Color::from_rgba(163, 62, 57, 255);
const HAIR_RED: Color = Color::from_rgba(109, 0, 22, 255);
const HAIR_DARKRED: Color = Color::from_rgba(78, 0, 19, 255);
const WOLF_WHITE: Color = Color::from_rgba(255, 255, 255, 255);
const WOLF_SHADOW_WHITE: Color = Color::from_rgba(178, 191, 223, 255);
const KNEES_DARKGREY: Color = Color::from_rgba(47, 47, 47, 255);
const BOOTS_BLACK: Color = Color::from_rgba(24, 24, 24, 255);
const ARMOR_GREY: Color = Color::from_rgba(120, 120, 120, 255);
const TAIL_BLUE: Color = Color::from_rgba(60, 64, 76, 255);

#[derive(Clone, PartialEq)]
pub enum Message {
    Restart,
}

pub enum State {
    PlayerDead,
    EnemiesDead,
    Playing,
}

pub fn build_style() -> Style {
    let style = Style {
        font_size: FONT_SIZE,
        layout: Layout::vertical(Vertical::Bottom, Horizontal::Center),
        pad: Pad::new(24.0, 4.0),
        // margin,
        // font,
        coloring: Coloring {
            at_rest: StateColor {
                bg_color: HAIR_RED,
                text_color: WOLF_WHITE,
                border_color: ARMOR_GREY,
            },
            hovered: StateColor {
                bg_color: HAIR_RED,
                text_color: WOLF_WHITE,
                border_color: WOLF_WHITE,
            },
            pressed: StateColor {
                bg_color: HAIR_DARKRED,
                text_color: WOLF_SHADOW_WHITE,
                border_color: BOOTS_BLACK,
            },
        },
        ..Default::default()
    };
    style
}

pub fn build_ui(style: &Style, screen: Vec2, state: State) -> Box<dyn RenderableWidget<Message>>{
    
    let mut ui = 
    // Container::new(style, vec![
        Button::new_text(style, Message::Restart, "Restart")
    // ])
    ;
    let layout = Layout::vertical(Vertical::Center, Horizontal::Center);
    
    let screen_rect = to_rect(vec2(0.0, 0.0), screen);
    compute_layout(&mut *ui, screen_rect, layout);
    ui
}


pub async fn render_loading_screen(done: i32, total: i32) {
    let screen = vec2(screen_width(), screen_height());
    let rect = add_contour(to_rect(Vec2::ZERO, screen), -screen * vec2(0.2, 0.49));
    let mut rect_progress = rect;
    rect_progress.w = rect_progress.w * done as f32 / total as f32;
    clear_background(BLACK);
    draw_rect(rect_progress, HAIR_RED);
    draw_rect_lines(rect, 2.0, ARMOR_GREY);
    next_frame().await;
}
