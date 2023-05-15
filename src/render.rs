use macroquad::prelude::*;

use crate::{behavior::parse_level::{WADLevelVertex, WADLevelLinedef}, vector::Vector2, MapTransform};
pub use crate::player::Player;

pub fn render(wall: Texture2D, player: &Player, verts: &Vec<WADLevelVertex>, linedefs: &Vec<WADLevelLinedef>, transform: MapTransform, mesh: &Mesh) {
    set_camera(&Camera3D {
        position: player.position,
        up: player.up,
        target: player.position.clone() + player.front.clone(),
        ..Default::default()
    });

    draw_grid(20, 1., BLACK, GRAY);

    // draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
    // draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
    // draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);
    draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), wall, WHITE);
    for l in linedefs {
        let v1i = verts[l.from as usize];
        let v1: Vector2<f32> = Vector2 { x: (v1i.x as f32 / 128. + transform.x_pos) * transform.scale, y: (v1i.y as f32 / 128. + transform.height) * transform.scale};
        let v2i = verts[l.to as usize];
        let v2: Vector2<f32> = Vector2 { x: (v2i.x as f32 / 128. + transform.x_pos) * transform.scale, y: (v2i.y as f32 / 128. + transform.height) * transform.scale };
        draw_line(v1.x, v1.y, v2.x, v2.y, 0.01, GREEN);
    }
    draw_mesh(mesh);
    set_default_camera();
}