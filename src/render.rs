use macroquad::prelude::*;

pub use crate::player::Player;

pub fn render(wall: Texture2D, player: &Player) {
    set_camera(&Camera3D {
        position: player.position,
        up: player.up,
        target: player.position.clone() + player.front.clone(),
        ..Default::default()
    });

    draw_grid(20, 1., BLACK, GRAY);

    draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
    draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
    draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);
    draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), wall, WHITE);

    set_default_camera();
}