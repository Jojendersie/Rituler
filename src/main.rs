extern crate sdl2;
extern crate sdl2_image;

//includes
mod math;
mod actor;
mod world;
mod playerinput;

//use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
use sdl2_image::LoadTexture;
use std::path::Path;

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
	let window = video_sys.window("Rituler", 1600, 900).build().ok().expect("Failed to create window.");
	let mut renderer = window.renderer().build().ok().expect("Failed to create renderer.");
	
	// Resources TODO: move to structures
	//let texture = renderer.load_texture(&Path::new("img/mage.png")).unwrap();
	
	//test
	let actor = actor::Actor::new( math::Vector{x : 10.0, y : 10.0}, renderer.load_texture(&Path::new("img/mage.png")).unwrap() );
	let actor2 = actor::Actor::new( math::Vector{x : 200.0, y : 420.0}, renderer.load_texture(&Path::new("img/golem.png")).unwrap() );
	let mut world = world::World{m_groundTiles : Vec::new(), m_gameObjects : Vec::new()};
	world.addActor(actor);
	world.addActor(actor2);
	
	while unsafe{running} {
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		// Handle all sdl events.
		for event in event_pump.poll_iter() {
			handle_event(event);
		}
		playerinput::handle_player_input(&sdl_context, &event_pump.keyboard_state(), &mut world.m_gameObjects[0]);
		
		renderer.clear();
		(&world as &actor::Drawable).draw(&mut renderer);
		renderer.present();
	}
}