extern crate sdl2;

use actor;
use std::f32;
use sdl2::keyboard::Scancode;
use math;

pub fn handle_player_input(_sdl_context: &sdl2::Sdl, _keyboard: &sdl2::keyboard::KeyboardState, _player: &mut actor::Actor) {
	//use sdl2::event::Event;
	//use sdl2::keyboard::Keycode;
	/*match event {
		Event::KeyDown { keycode: Some(Keycode::W), .. } => {
			println!("W");
			_player.m_location.x += 2;
		},
		_ => {}
	}*/
	//_player.m_location.x += 2.0;
	
	let (_, mx, my) = _sdl_context.mouse().mouse_state();
	_player.m_angle = f32::atan2((my-(_player.m_location.y as i32)-(_player.m_sprite_size.1 as i32)) as f32,
								 (mx-(_player.m_location.x as i32)-(_player.m_sprite_size.0 as i32)) as f32) * 180.0 / f32::consts::PI + 45.0;
	
	let mut move_dir = math::Vector{x:0.0, y:0.0};
	if _keyboard.is_scancode_pressed(Scancode::W) {
		move_dir.y -= 1.0;
	}
	if _keyboard.is_scancode_pressed(Scancode::S) {
		move_dir.y += 1.0;
	}
	if _keyboard.is_scancode_pressed(Scancode::D) {
		move_dir.x += 1.0;
	}
	if _keyboard.is_scancode_pressed(Scancode::A) {
		move_dir.x -= 1.0;
	}
	move_dir.normalize();
	
	//let old_pos = _player.m_location.clone();
	_player.m_location = _player.m_location + move_dir;
}