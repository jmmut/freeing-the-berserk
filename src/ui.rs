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
    let mut style = build_button_style(true);
    // style.coloring.at_rest.bg_color = ARMOR_GREY;
    // style.coloring.at_rest.text_color = WOLF_WHITE;
    style.pad.y = style.pad.x;
    style
}
pub fn build_button_style(enabled: bool) -> Style {
    let text_color = if enabled { WOLF_WHITE } else { ARMOR_GREY };
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
                text_color,
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

pub fn build_loading_style() -> Style {
    let state_color = StateColor {
        bg_color: BLACK,
        text_color: WOLF_WHITE,
        border_color: BLACK,
    };
    let mut style = build_button_style(true);
    style.coloring = Coloring {
        at_rest: state_color,
        hovered: state_color,
        pressed: state_color,
    };
    style
}

pub fn build_ui(screen: Vec2, state: State) -> Box<dyn RenderableWidget<Message>> {
    let button_style = build_button_style(true);
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

pub fn build_loading_ui(screen: Vec2, all_loaded: bool) -> Box<dyn RenderableWidget<Message>> {
    let button_style = build_button_style(all_loaded);
    let loading_style = build_loading_style();
    let mut left_aligned_style = loading_style.clone();
    left_aligned_style.layout = Layout::vertical(Vertical::Bottom, Horizontal::Left);
    left_aligned_style.pad = Pad::new(0.0, 0.0);

    let mut title_style = loading_style.clone();
    title_style.coloring.at_rest.text_color = HAIR_RED;
    title_style.font_size *= 4.0;
    title_style.pad = Pad::new(0.0, 40.0);

    let mut ui = Container::new(
        &loading_style,
        vec![
            Text::new(&title_style, "Freeing the Berserk"),
            Text::new(&loading_style, "Controls"),
            Container::new(
                &left_aligned_style,
                vec![
                    Text::new(&left_aligned_style, "WASD or keyboard arrows to move"),
                    Text::new(&left_aligned_style, "Space or J to attack"),
                    Text::new(&left_aligned_style, "Shift or K to dash"),
                ],
            ),
            Button::new_text(&button_style, Message::Restart, "Play"),
        ],
    );

    let layout = Layout::vertical(Vertical::Center, Horizontal::Center);
    let screen_rect = to_rect(vec2(0.0, 0.0), screen);
    compute_layout(&mut *ui, screen_rect, layout);
    ui
}

/// returns True if the rendering loop should continue in the loading screen.
pub async fn render_loading_screen(done: i32, total: i32) -> bool {
    let screen = vec2(screen_width(), screen_height());
    let mut rect = add_contour(to_rect(Vec2::ZERO, screen), -screen * vec2(0.2, 0.49));
    rect.y = screen.y * 0.8;
    let mut rect_progress = rect;
    rect_progress.w = rect_progress.w * done as f32 / total as f32;
    clear_background(BLACK);

    let all_loaded = done == total;
    let mut ui = build_loading_ui(screen, all_loaded);
    let messages = if all_loaded { ui.interact() } else { vec![] };
    ui.render();

    draw_rect(rect_progress, HAIR_RED);
    draw_rect_lines(rect, 2.0, ARMOR_GREY);
    next_frame().await;
    let should_continue = !messages.contains(&Message::Restart);
    should_continue
}
