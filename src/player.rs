use macroquad::prelude::*;
pub struct Player {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub grabbed: bool,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub last_mouse_position: Vec2
}