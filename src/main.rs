extern crate sdl2;
extern crate sdl2_image;

//includes
mod math;
mod actor;
mod world;

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
			_ => {}
	}
}

fn main() {
	let sdl_context = sdl2::init().ok().expect("Failed to create context.");
	let video_sys = sdl_context.video().ok().expect("Failed to create video context.");
	let _image_context = sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);
	let window = video_sys.window("Rituler", 1600, 900).build().ok().expect("Failed to create window.");
	let mut renderer = window.renderer().build().ok().expect("Failed to create renderer.");
	
	// Resources TODO: move to structures
	let texture = renderer.load_texture(&Path::new("img/mage.png")).unwrap();
	
	//test
	let mut world = world::World{m_groundTiles : Vec::new(), m_gameObjects : Vec::new()};
	let mut actor = actor::Actor{m_location : math::Vector{x : 10.0, y : 10.0}, m_texture : renderer.load_texture(&Path::new("img/mage.png")).unwrap()};
	let mut actor2 = actor::Actor{m_location : math::Vector{x : 20.0, y : 42.0}, m_texture : renderer.load_texture(&Path::new("img/golem.png")).unwrap()};
	world.addActor(actor);
	world.addActor(actor2);
	
	while unsafe{running} {
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		// Handle all sdl events.
		for event in event_pump.poll_iter() {
			handle_event(event);
		}
		
		//let mx:i32;
		//let my:i32;
		let (_, mx, my) = sdl_context.mouse().mouse_state();
		let angle = f32::atan2((my-160) as f32, (mx-162) as f32) * 180.0 / f32::consts::PI + 45.0;
		renderer.clear();
		(&world as &actor::Drawable).draw(&mut renderer);
		renderer.present();
	}
}