extern crate sdl2;
extern crate sdl2_image;

//includes
mod constants;
mod math;
mod actor;
mod world;
mod playerinput;

//use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
use sdl2_image::LoadTexture;
use std::path::Path;
use sdl2::rect::{Point};
use constants::*;

static mut running: bool = true;

fn handle_event(event: sdl2::event::Event) {
	use sdl2::event::Event;
	use sdl2::keyboard::Keycode;
	match event {
		Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => unsafe {
				running = false;
		},
		_ => {
		}
	}
}

fn main() {
	let sdl_context = sdl2::init().ok().expect("Failed to create context.");
	let video_sys = sdl_context.video().ok().expect("Failed to create video context.");
	let _image_context = sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);
	let window = video_sys.window("Rituler", WIN_WIDTH as u32, WIN_HEIGHT as u32).build().ok().expect("Failed to create window.");
	let mut renderer = window.renderer().build().ok().expect("Failed to create renderer.");
	
	// the "textureManager"
	let mut textures : Vec < sdl2::render::Texture > = Vec::new();
	
	//load resources
	textures.push(renderer.load_texture(&Path::new("img/mage.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/golem.png")).unwrap());
	//ground layer
	textures.push(renderer.load_texture(&Path::new("img/grass.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/sand.png")).unwrap());
	//test
	let actor = actor::Actor::new( math::Vector{x : 10.0, y : 10.0},  &textures[0]);
	let actor2 = actor::Actor::new( math::Vector{x : 0.0, y : 0.0}, &textures[1] );
	let mut world = world::World::new(vec![&textures[2], &textures[3]]);//world::World{m_groundTiles : Vec::new(), m_gameObjects : Vec::new()};
	world.addActor(actor);
	world.addActor(actor2);
	
	while unsafe{running} {
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		// Handle all sdl events.
		for event in event_pump.poll_iter() {
			handle_event(event);
		}
		playerinput::handle_player_input(&sdl_context, &event_pump.keyboard_state(), &mut world.m_gameObjects[0]);
		
		// The camera is always attached to the player which is entity 0
		let cam_pos = Point::new(world.m_gameObjects[0].m_location.x as i32 - WIN_WIDTH/2, world.m_gameObjects[0].m_location.y as i32 - WIN_HEIGHT/2);
		
		renderer.clear();
		(&world as &actor::Drawable).draw(&mut renderer, &cam_pos);
		renderer.present();
	}
}