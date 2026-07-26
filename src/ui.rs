use crate::{add_contour, to_rect};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::elm::button::Button;
use juquad::elm::container::Container;
use juquad::elm::style::Style;
use juquad::elm::text::Text;
use juquad::elm::widget::{compute_layout, RenderableWidget};
use juquad::lazy::{Margin, Pad};
use juquad::widgets::anchor::{Horizontal, Layout, Vertical};
use juquad::widgets::{Coloring, StateColor};
use macroquad::color::{Color, BLACK};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width};

pub const FONT_SIZE: f32 = 16.0;

pub const EYES_RED: Color = Color::from_rgba(239, 17, 17, 255);
pub const BELT_LIGHTRED: Color = Color::from_rgba(163, 62, 57, 255);
pub const HAIR_RED: Color = Color::from_rgba(109, 0, 22, 255);
pub const HAIR_DARKRED: Color = Color::from_rgba(78, 0, 19, 255);
pub const WOLF_WHITE: Color = Color::from_rgba(255, 255, 255, 255);
pub const ARMOR_GREY: Color = Color::from_rgba(120, 120, 120, 255);
pub const SHIELD_GREY: Color = Color::from_rgba(120, 120, 120, 255);
pub const KNEES_DARKGREY: Color = Color::from_rgba(47, 47, 47, 255);
pub const BOOTS_BLACK: Color = Color::from_rgba(24, 24, 24, 255);
pub const WOLF_SHADOW_WHITE: Color = Color::from_rgba(178, 191, 223, 255);
pub const TAIL_BLUE: Color = Color::from_rgba(60, 64, 76, 255);

#[derive(Clone, PartialEq)]
pub enum Message {
    Restart,
}

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    PlayerDead,
    EnemiesDead,
    Playing,
}

pub fn build_panel_style() -> Style {
    let mut style = build_button_style();
    // style.coloring.at_rest.bg_color = ARMOR_GREY;
    // style.coloring.at_rest.text_color = WOLF_WHITE;
    style.pad.y = style.pad.x;
    style
}
pub fn build_button_style() -> Style {
    let style = Style {
        font_size: FONT_SIZE,
        layout: Layout::vertical(Vertical::Bottom, Horizontal::Center),
        pad: Pad::new(24.0, 4.0),
        margin: Margin::new(10.0, 10.0),
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
                text_color: EYES_RED,
                border_color: BOOTS_BLACK,
            },
        },
        ..Default::default()
    };
    style
}

pub fn build_ui(screen: Vec2, state: State) -> Box<dyn RenderableWidget<Message>> {
    let button_style = build_button_style();
    let panel_style = build_panel_style();

    let mut contents = Vec::new();
    match state {
        State::PlayerDead => contents.push(Text::new(&button_style, "You died.")),
        State::EnemiesDead => contents.push(Text::new(&button_style, "You won.")),
        State::Playing => {}
    };
    contents.push(Button::new_text(
        &button_style,
        Message::Restart,
        "Restart (R)",
    ));
    let mut ui = Container::new(panel_style, contents);

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
