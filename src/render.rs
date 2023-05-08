use macroquad::prelude::*;

use crate::behavior::parse_level::{WADLevelVertex, WADLevelLinedef};
pub use crate::player::Player;

pub fn render(wall: Texture2D, player: &Player, verts: &Vec<WADLevelVertex>, linedefs: &Vec<WADLevelLinedef>) {
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
    for l in linedefs {
        let v1 = verts[l.from as usize];
        let v2 = verts[l.to as usize];
        // draw_line(v1.x as f32 / 65536., v1.y as f32 / 65536., v2.x as f32 / 65536., v2.y as f32 / 65536., 1., GREEN);
        // println!("Drawing line from: {},{} to: {},{}", v1.x as f32 / 65536., v1.y as f32 / 65536., v2.x as f32 / 65536., v2.y as f32 / 65536.);
        draw_line(v1.x as f32 / 128., v1.y as f32 / 128., v2.x as f32 / 128., v2.y as f32 / 128., 0.01, GREEN);
        // println!("Drawing line from: {},{} to: {},{}", v1.x, v1.y, v2.x, v2.y);
        // let vec1: Vec3 = Vec3 { x: v1.x as f32 / 2048., y: 0., z: v1.y as f32 / 2048. };
        // let vec2: Vec3 = Vec3 { x: v2.x as f32 / 2048., y: 0., z: v2.y as f32 / 2048. };
        // draw_line_3d(vec1, vec2, GREEN);
    }
    set_default_camera();
}