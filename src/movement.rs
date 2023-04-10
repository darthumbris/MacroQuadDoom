use macroquad::prelude::*;


use crate::player::Player;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.2;

pub fn movement(delta: f32, player: &mut Player) {
    let world_up = vec3(0.0, 1.0, 0.0);

    if is_key_pressed(KeyCode::Tab) {
        player.grabbed = !player.grabbed;
        set_cursor_grab(player.grabbed);
        show_mouse(!player.grabbed);
    }

    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        player.position += player.front * MOVE_SPEED;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        player.position -= player.front * MOVE_SPEED;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        player.position -= player.right * MOVE_SPEED;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        player.position += player.right * MOVE_SPEED;
    }

    if is_key_down(KeyCode::Q) {
        player.yaw -= 7. * delta * LOOK_SPEED;
    }

    if is_key_down(KeyCode::E) {
        player.yaw += 7. * delta * LOOK_SPEED;
    }

    let mouse_position: Vec2 = mouse_position().into();
    let mouse_delta = mouse_position - player.last_mouse_position;
    player.last_mouse_position = mouse_position;

    player.yaw += mouse_delta.x * delta * LOOK_SPEED;
    player.pitch += mouse_delta.y * delta * -LOOK_SPEED;

    player.pitch = if player.pitch > 1.5 { 1.5 } else { player.pitch };
    player.pitch = if player.pitch < -1.5 { -1.5 } else { player.pitch };

    player.front = vec3(
        player.yaw.cos() * player.pitch.cos(),
        player.pitch.sin(),
        player.yaw.sin() * player.pitch.cos(),
    )
    .normalize();

    player.right = player.front.cross(world_up).normalize();
    player.up = player.right.cross(player.front).normalize();
}