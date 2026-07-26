use crate::AnyResult;
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use crate::loader::Loader;

pub struct Textures {
    pub player: Vec<Animations>,
    pub enemies: Animations,
    pub background: Vec<Texture2D>,
    pub overlay: Vec<Texture2D>,
    pub character_size: Vec2,
}

#[derive(Clone)]
pub struct Animations {
    pub attack: Vec<Texture2D>,
    pub dash: Vec<Texture2D>,
    pub idle: Vec<Texture2D>,
    pub walk: Vec<Texture2D>,
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
                walk: vec![],
                attack: vec![],
                dash: vec![],
            }],
            enemies: Animations {
                idle: vec![],
                walk: vec![],
                attack: vec![],
                dash: vec![],
            },
            background: vec![],
            overlay: vec![],
            character_size: Vec2::ONE,
        }
    }
    pub async fn load(loader: &mut Loader) -> AnyResult<Self> {
        let player = load_player(loader).await?;
        let enemies = player[5].clone();
        Ok(Self {
            player,
            enemies,
            background: load_single_v("other-sprites/bg", loader).await?,
            overlay: load_single_v("other-sprites/overlay", loader).await?,
            character_size: Vec2::new(2.0, 1.0),
        })
    }
}

pub async fn load_player(loader: &mut Loader) -> AnyResult<Vec<Animations>> {
    let mut player = Vec::new();
    for i in 1..=6 {
        let moving = load(&format!("chara-sprites/chara{}-walking", i), 2, loader).await?;
        let dashing = if i <= 2 {
            load_single_v(&format!("chara-sprites/chara{}-dash", i), loader).await?
        } else {
            vec![moving[0].clone()]
        };
        player.push(Animations {
            idle: load_single_v(&format!("chara-sprites/chara{}-idle", i), loader).await?,
            walk: moving,
            attack: load_single_v(&format!("chara-sprites/chara{}-attack", i), loader).await?,
            dash: dashing,
        })
    }
    Ok(player)
}

pub async fn load(path: &str, count: usize, loader: &mut Loader) -> AnyResult<Vec<Texture2D>> {
    let mut textures = Vec::new();
    for i in 1..=count {
        let path = format!("{}_{:>02}", path, i);
        textures.push(load_single(&path, loader).await?);
    }
    Ok(textures)
}
pub async fn load_single_v(path: &str, loader: &mut Loader) -> AnyResult<Vec<Texture2D>> {
    Ok(vec![load_single(path, loader).await?])
}
pub async fn load_single(path: &str, loader: &mut Loader) -> AnyResult<Texture2D> {
    let path = format!("assets/images/{}.png", path);
    eprintln!("loading {}", path);
    loader.load_texture(path).await
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
