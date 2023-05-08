use macroquad::prelude::*;

pub mod movement;
pub mod render;
pub mod player;
pub mod parser;
pub mod level;
pub mod vector;

pub mod behavior;

pub use movement::movement;
pub use render::render;
pub use player::Player;
pub use parser::parse_map;

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

    let wad = parse_map("Assets/DOOM1.WAD");

    let wall = load_texture("Assets/greystone_128.png").await.unwrap();

    let mut player = init_world();

    loop {
        let delta = get_frame_time();
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        movement(delta, &mut player);

        render(wall, & player, &wad.levels[8].vertexes, &wad.levels[8].linedefs);

        next_frame().await
    }
}