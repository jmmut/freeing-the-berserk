use crate::AnyResult;
use macroquad::math::Vec2;
use macroquad::prelude::{load_texture, Texture2D};

pub struct Textures {
    pub player: Vec<Animations>,
    pub enemies: Animations,
    pub background: Vec<Texture2D>,
    pub character_size: Vec2,
}

pub struct Animations {
    pub idle: Vec<Texture2D>,
    pub moving: Vec<Texture2D>,
    pub attacking: Vec<Texture2D>,
    pub dashing: Vec<Texture2D>,
}

pub struct Animator {
    time_s: f64,
}

impl Textures {
    #[allow(unused)]
    pub fn new_empty() -> Self {
        Self {
            player: vec![Animations {
                idle: vec![],
                moving: vec![],
                attacking: vec![],
                dashing: vec![],
            }],
            enemies: Animations {
                idle: vec![],
                moving: vec![],
                attacking: vec![],
                dashing: vec![],
            },
            background: vec![],
            character_size: Vec2::ONE,
        }
    }
    pub async fn load() -> AnyResult<Self> {
        Ok(Self {
            player: vec![
                Animations {
                    idle: vec![load_single("chara-sprites/chara0-idle").await?],
                    moving: load("chara-sprites/chara0-walking", 2).await?,
                    attacking: vec![load_single("chara-sprites/chara0-attack").await?],
                    dashing: vec![load_single("chara-sprites/chara0-dash").await?],
                },
                Animations {
                    idle: vec![load_single("chara-sprites/chara1-idle").await?],
                    moving: load("chara-sprites/chara1-walking", 2).await?,
                    attacking: vec![load_single("chara-sprites/chara1-attack").await?],
                    dashing: vec![load_single("chara-sprites/chara1-walking_01").await?],
                },
            ],
            enemies: Animations {
                idle: vec![load_single("chara-sprites/chara1-idle").await?], // TODO
                // moving: load("chara-sprites/chara0-walking", 2).await?, // TODO
                moving: load("chara-sprites/chara1-walking", 2).await?, // TODO
                attacking: vec![load_single("chara-sprites/chara1-attack").await?], // TODO
                dashing: vec![load_single("chara-sprites/chara1-walking_01").await?],
            },
            background: vec![load_single("other-sprites/bg").await?],
            character_size: Vec2::new(2.0, 1.0),
        })
    }
}

pub async fn load(path: &str, count: usize) -> AnyResult<Vec<Texture2D>> {
    let mut textures = Vec::new();
    for i in 1..=count {
        let path = format!("{}_{:>02}", path, i);
        textures.push(load_single(&path).await?);
    }
    Ok(textures)
}
pub async fn load_single(path: &str) -> AnyResult<Texture2D> {
    let path = format!("assets/images/{}.png", path);
    eprintln!("loading {}", path);
    Ok(load_texture(&path).await?)
}

impl Animator {
    /// around 1e6, divisible by many numbers, so that when it wraps, it removes an integer number
    /// of animation cycles.
    const TIME_CYCLE: f64 = 2.0 * 3.0 * 5.0 * 7.0 * 11.0 * 500.0;

    const ANIMATION_FPS: f64 = 4.0;

    pub fn new() -> Self {
        Self { time_s: 0.0 }
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.time_s += delta_s;
        if self.time_s > Self::TIME_CYCLE {
            self.time_s -= Self::TIME_CYCLE;
        }
    }
    pub fn choose_texture<'a>(&self, animation: &'a Vec<Texture2D>) -> &'a Texture2D {
        let total_frame_index = self.time_s * Self::ANIMATION_FPS;
        let frame_index = total_frame_index as usize % animation.len();
        &animation[frame_index]
    }
}
