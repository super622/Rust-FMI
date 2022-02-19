pub use glam::*;

pub const SHOT_SPEED: f32 = 250.0;
pub const PLAYER_TURN: f32 = 3.0;
pub const PLAYER_ACCELERATION: f32 = 50.0;
pub const SHOT_DELAY: f32 = 0.5;
pub const SHOT_LIFE: f32 = 4.0;
pub const SHOT_BBOX: f32 = 6.0;
pub const JET_HBOX: f32 = 7.0;
pub const MAX_PHYSICS_VEL: f32 = 150.0;
pub const DESIRED_FPS: u32 = 60;
pub const SHOT_ANG_VEL: f32 = 0.1;

pub struct Jet {
    pub position: Vec2,
    pub facing: f32,
    pub striked: u16,
    pub speed: Vec2,
    pub hitbox: f32,
    pub ang_vel: f32,
    pub shot_timeout: f32,
}

impl Jet {
    pub fn new() -> Jet{
        Jet{
            position: Vec2::ZERO,
            facing: 0.0,
            striked: 0,
            speed: Vec2::ZERO,
            hitbox: JET_HBOX,
            ang_vel: 0.,
            shot_timeout: SHOT_DELAY,
        }
    }
}

pub struct Shot{
    pub position: Vec2,
    pub facing: f32,
    pub speed: Vec2,
    pub ang_vel: f32,
    pub shotbox: f32,
    pub life: f32,
}

impl Shot {
    pub fn new() -> Shot{
        Shot{
            position: Vec2::ZERO,
            facing: 0.0,
            speed: Vec2::ZERO,
            ang_vel: SHOT_ANG_VEL,
            shotbox: SHOT_BBOX,
            life: SHOT_LIFE,
        }
    }
}

#[derive(Debug)]
pub struct InputState {
    pub x: f32,
    pub y: f32,
    pub fire: bool,
}

impl Default for InputState{
    fn default() -> Self{
        InputState{
            x: 0.0,
            y: 0.0,
            fire:false,
        }
    }
}