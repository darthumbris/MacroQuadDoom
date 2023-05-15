use macroquad::prelude::*;

pub mod movement;
pub mod render;
pub mod player;
pub mod parser;
pub mod level;
pub mod vector;
pub mod game;

pub mod behavior;
pub mod file_system;

pub use movement::movement;
pub use render::render;
pub use player::Player;
pub use parser::parse_map;

use crate::level::level_load::MapLoader;
use crate::game::Game;
use crate::level::level_texture::TextureManager;
use crate::level::LevelLocals;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Doom"),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

fn init_world()->Player {
    let world_up = vec3(0.0, 1.0, 0.0);
    let yaw:f32 = 1.18;
    let pitch:f32 = 0.0;
    let front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let player = Player {
        yaw: 1.18,
        pitch: 0.0,
        front: vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize(),
        right: front.cross(world_up).normalize(),
        up: Vec3 { x: 0., y: 0., z: 0. },
        position: vec3(0.0, 1.0, 0.0),
        grabbed: true,
        last_mouse_position: mouse_position().into()
    };
    set_cursor_grab(player.grabbed);
    show_mouse(false);
    player
}

#[macroquad::main(conf)]
async fn main() {

    let mut wad = parse_map("Assets/DOOM1.WAD");

    let wall = load_texture("Assets/greystone_128.png").await.unwrap();

    let mut player = init_world();
    let mut level_layer: i32 = 0;
    let mut height: f32 = 0.;
    let mut x_pos: f32 = 0.;
    let mut scale: f32 = 1.;
    println!("Making Game");
    let game = Game::new();
    println!("Made Game");
    let mut level = LevelLocals::default();
    println!("Made Level");
    let tex_manager = TextureManager::new();
    println!("Made Texture Manager");
    let mut maploader: MapLoader = MapLoader::new(&mut level, &tex_manager);
    println!("Made MapLoader");
    maploader.load_level(&mut wad.levels[0], &game);
    println!("Loaded level");
    // println!("levelmesh: vertexes len: {}, indices: {:?}", level.level_mesh.vertices.len(), level.level_mesh.uv_index);
    // println!("levelmesh: vertexes: {:?}", level.level_mesh.vertices);
    println!("level: {}", level_layer);

    // let mesh: Mesh = Mesh { vertices: level.level_mesh.vertices, indices: level.level_mesh.uv_index, texture: None };
    let mesh: Mesh = level.level_mesh.to_macro_mesh(wall);
    loop {
        let delta = get_frame_time();
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_released(KeyCode::KpAdd) {
            height = 0.;
            x_pos = 0.;
            level_layer += 1;
            if level_layer > (wad.levels.len() - 1) as i32 {level_layer = 0}
            println!("level: {}", level_layer);
        }
    
        if is_key_released(KeyCode::KpSubtract) {
            height = 0.;
            level_layer -= 1;
            x_pos = 0.;
            if level_layer < 0 {level_layer = (wad.levels.len() - 1) as i32}
            println!("level: {}", level_layer);
        }

        if is_key_down(KeyCode::Kp8) {height += 1.}
        if is_key_down(KeyCode::Kp2) {height -= 1.}
        if is_key_down(KeyCode::Kp4) {x_pos += 1.}
        if is_key_down(KeyCode::Kp6) {x_pos -= 1.}
        if is_key_released(KeyCode::KpMultiply) {scale *= 1.25}
        if is_key_released(KeyCode::KpDivide) {scale /= 1.25}

        
        
        
        movement(delta, &mut player);
        // println!("level: {}", level);
        render(wall, & player, &wad.levels[level_layer as usize].vertexes, &wad.levels[level_layer as usize].linedefs, MapTransform{height, x_pos, scale}, &mesh);

        next_frame().await
    }
}

pub struct MapTransform {
    height: f32,
    x_pos: f32,
    scale: f32
}